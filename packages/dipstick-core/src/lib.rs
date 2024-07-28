use std::borrow::Borrow;
use std::sync::Arc;

use dipstick_proto::core::v1::{
    CoreService, CoreServiceServer, EntityMetaSpec, EntitySelector, File, FileVariant,
    ForceRemoveAllEntitiesRequest, ForceRemoveAllEntitiesResponse, ListEntitiesRequest,
    ListEntitiesResponse, LogConfigRequest, LogConfigResponse, LogSubscribeRequest,
    LogSubscribeResponse, ShutdownRequest, ShutdownResponse, VersionRequest, VersionResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Result, Status};

pub use self::dependency::{Dep, DependencyHandle};
pub use self::entity::{Entity, EntityKind, EntityMeta, QualifiedKey, UniqueId};
pub use self::package::{Package, PackageKind};
pub use self::registry::ReservationHandle;

mod dependency;
mod entity;
pub mod logging;
mod package;
mod registry;

pub struct Core {
    registry: registry::Registry,
    log_handle: logging::LoggingHandle,
    shutdown_tx: mpsc::Sender<()>,
    version: String,
}

impl Core {
    pub fn new(
        version: String,
        log_handle: logging::LoggingHandle,
        shutdown_tx: mpsc::Sender<()>,
    ) -> Arc<Self> {
        let registry = registry::Registry::new();
        Arc::new(Self {
            registry,
            log_handle,
            shutdown_tx,
            version,
        })
    }

    pub fn into_server(self: Arc<Self>) -> CoreServiceServer<Self> {
        CoreServiceServer::from_arc(self)
    }

    pub fn add_package<T: Package>(&self, package: Arc<T>) {
        self.registry.add_package(package);
    }

    pub fn get_package<T: Package + PackageKind>(&self) -> Result<Arc<T>> {
        self.registry.package()
    }

    pub fn new_entity_meta<T: EntityKind>(
        &self,
        mut spec: EntityMetaSpec,
    ) -> Result<(EntityMeta, ReservationHandle)> {
        spec.package = T::Package::PACKAGE_NAME.to_owned();
        spec.kind = T::KIND.to_owned();
        self.new_entity_meta_raw(spec)
    }

    pub fn add_entity<T: Entity>(&self, reservation: ReservationHandle, entity: Arc<T>) {
        self.registry.add_entity(reservation, entity);
    }

    pub fn select_entity<T: EntityKind + Entity>(
        &self,
        selector: impl Borrow<EntitySelector>,
    ) -> Result<Arc<T>> {
        self.registry.select(selector.borrow())
    }

    pub fn select_entity_dep<T: EntityKind + Entity>(
        &self,
        dependent: &impl Entity,
        selector: impl Borrow<EntitySelector>,
    ) -> Result<Dep<T>> {
        let entity = self.select_entity::<T>(selector)?;
        Ok(Dep::new(dependent.entity_meta(), entity))
    }

    fn new_entity_meta_raw(&self, spec: EntityMetaSpec) -> Result<(EntityMeta, ReservationHandle)> {
        assert!(!spec.package.is_empty() && !spec.kind.is_empty());
        let reservation = self.registry.reserve(
            UniqueId::new(spec.unique_id),
            QualifiedKey::from_spec(&spec),
        )?;
        let meta = EntityMeta::new(reservation.unique_id(), spec);
        Ok((meta, reservation))
    }

    pub async fn read_file_to_string(&self, file: &File) -> Result<String> {
        match &file.file_variant {
            Some(FileVariant::ServerPath(path)) => {
                let current_dir = std::env::current_dir()?; // TODO
                let path = current_dir.join(path).canonicalize()?;
                if !path.starts_with(current_dir) {
                    return Err(Status::invalid_argument("invalid server path"));
                }
                tracing::debug!(?path, "loading server path file");
                let content = tokio::fs::read_to_string(path).await?;
                Ok(content)
            }
            None => Err(Status::invalid_argument("missing file variant")),
        }
    }
}

impl CoreService for Core {
    fn shutdown<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<ShutdownRequest>,
    ) -> BoxFuture<'fut, Result<Response<ShutdownResponse>>> {
        async move {
            tracing::info!("received shutdown request");
            let _ = self.shutdown_tx.send(()).await;
            Ok(Response::new(ShutdownResponse {}))
        }
        .boxed()
    }

    fn version<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<VersionRequest>,
    ) -> BoxFuture<'fut, Result<Response<VersionResponse>>> {
        async move {
            Ok(Response::new(VersionResponse {
                version: self.version.clone(),
            }))
        }
        .boxed()
    }

    fn log_config<'s: 'fut, 'fut>(
        &'s self,
        request: Request<LogConfigRequest>,
    ) -> BoxFuture<'fut, Result<Response<LogConfigResponse>>> {
        async move {
            let LogConfigRequest { config } = request.into_inner();
            if let Some(config) = config {
                // update config
                self.log_handle
                    .set_log_config(config)
                    .map_err(|err| Status::internal(err.to_string()))?;
            }

            let Some(config) = self.log_handle.log_config() else {
                return Err(Status::internal("subscriber dropped"));
            };
            Ok(Response::new(LogConfigResponse {
                config: Some(config),
            }))
        }
        .boxed()
    }

    type LogSubscribeStream = BoxStream<'static, Result<LogSubscribeResponse>>;

    fn log_subscribe<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<LogSubscribeRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::LogSubscribeStream>>> {
        async move {
            let stream = BroadcastStream::new(self.log_handle.logs_tx.subscribe())
                .filter_map(|item| {
                    std::future::ready(match item {
                        Ok(event) => Some(Ok(LogSubscribeResponse { event: Some(event) })),
                        _ => None,
                    })
                })
                .boxed();

            Ok(Response::new(stream))
        }
        .boxed()
    }

    fn list_entities<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<ListEntitiesRequest>,
    ) -> BoxFuture<'fut, Result<Response<ListEntitiesResponse>>> {
        async move {
            let mut entities = Vec::new();
            self.registry
                .visit_all(|entity| entities.push(entity.entity_meta().to_proto()));
            Ok(Response::new(ListEntitiesResponse { entities }))
        }
        .boxed()
    }

    fn force_remove_all_entities<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<ForceRemoveAllEntitiesRequest>,
    ) -> BoxFuture<'fut, Result<Response<ForceRemoveAllEntitiesResponse>>> {
        async move {
            self.registry.force_remove_all_entities();
            Ok(Response::new(ForceRemoveAllEntitiesResponse {}))
        }
        .boxed()
    }
}

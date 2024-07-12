use std::sync::Arc;

use dipstick_proto::core::v1::{
    CoreService, CoreServiceServer, ListEntitiesRequest, ListEntitiesResponse, LogConfigRequest,
    LogConfigResponse, LogSubscribeRequest, LogSubscribeResponse, ShutdownRequest,
    ShutdownResponse, VersionRequest, VersionResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response};

pub use self::entity::{Entity, EntityMeta, EntitySelector, IdContext, QualifiedId, UniqueId};
pub use self::registry::Registry;

mod consts;
mod entity;
pub mod logging;
mod registry;

pub struct Core {
    registry: Arc<Registry>,
    log_handle: logging::LoggingHandle,
    shutdown_tx: mpsc::Sender<()>,
}

impl Core {
    pub fn new(log_handle: logging::LoggingHandle, shutdown_tx: mpsc::Sender<()>) -> Arc<Self> {
        let registry = Arc::new(Registry::new());
        Arc::new(Self {
            registry,
            log_handle,
            shutdown_tx,
        })
    }

    pub fn into_server(self: Arc<Self>) -> CoreServiceServer<Self> {
        CoreServiceServer::from_arc(self)
    }

    pub fn registry(&self) -> Arc<Registry> {
        Arc::clone(&self.registry)
    }
}

impl CoreService for Core {
    fn shutdown<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<ShutdownRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<ShutdownResponse>>> {
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
    ) -> BoxFuture<'fut, tonic::Result<Response<VersionResponse>>> {
        async move {
            Ok(Response::new(VersionResponse {
                version: crate::consts::VERSION.to_owned(),
            }))
        }
        .boxed()
    }

    fn log_config<'s: 'fut, 'fut>(
        &'s self,
        request: Request<LogConfigRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<LogConfigResponse>>> {
        async move {
            let LogConfigRequest { config } = request.into_inner();
            if let Some(config) = config {
                // update config
                self.log_handle
                    .set_log_config(config)
                    .map_err(|err| tonic::Status::internal(err.to_string()))?;
            }

            let Some(config) = self.log_handle.log_config() else {
                return Err(tonic::Status::internal("subscriber dropped"));
            };
            Ok(Response::new(LogConfigResponse {
                config: Some(config),
            }))
        }
        .boxed()
    }

    type LogSubscribeStream = BoxStream<'static, tonic::Result<LogSubscribeResponse>>;

    fn log_subscribe<'s: 'fut, 'fut>(
        &'s self,
        _request: Request<LogSubscribeRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<Self::LogSubscribeStream>>> {
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
    ) -> BoxFuture<'fut, tonic::Result<Response<ListEntitiesResponse>>> {
        async move {
            let mut entities = Vec::new();
            self.registry
                .visit_entities(|entity| entities.push(entity.entity_meta().to_proto()));
            Ok(Response::new(ListEntitiesResponse { entities }))
        }
        .boxed()
    }
}

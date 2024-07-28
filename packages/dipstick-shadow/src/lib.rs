use std::sync::Arc;

use dipstick_core::{Core, Package, PackageKind};
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::shadow::v1::{
    CreateShadowRequest, CreateShadowResponse, GetShadowRequest, GetShadowResponse,
    ShadowEventsRequest, ShadowEventsResponse, ShadowServiceServer, ShadowSpec,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tonic::{Request, Response, Result, Status};

pub use self::shadow::Shadow;

mod listeners;
mod shadow;

pub struct ShadowService {
    core: Arc<Core>,
}

impl ShadowService {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> ShadowServiceServer<Self> {
        ShadowServiceServer::from_arc(self)
    }

    pub async fn create_shadow_impl(
        &self,
        meta: EntityMetaSpec,
        spec: ShadowSpec,
    ) -> Result<Arc<Shadow>> {
        let (meta, reservation) = self.core.new_entity_meta::<Shadow>(meta)?;
        let shadow = Shadow::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&shadow));
        Ok(shadow)
    }
}

impl Package for ShadowService {
    fn package_name(&self) -> &'static str {
        Self::PACKAGE_NAME
    }
}

impl PackageKind for ShadowService {
    const PACKAGE_NAME: &'static str = "shadow.v1";
}

impl dipstick_proto::shadow::v1::ShadowService for ShadowService {
    fn create_shadow<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateShadowRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateShadowResponse>>> {
        async move {
            let CreateShadowRequest { meta, spec } = request.into_inner();
            let shadow = self
                .create_shadow_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateShadowResponse {
                shadow: Some(shadow.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_shadow<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetShadowRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetShadowResponse>>> {
        async move {
            let GetShadowRequest { selector } = request.into_inner();
            let shadow = self
                .core
                .select_entity::<Shadow>(selector.unwrap_or_default())?;
            Ok(Response::new(GetShadowResponse {
                shadow: Some(shadow.to_proto()),
            }))
        }
        .boxed()
    }

    type ShadowEventsStream = BoxStream<'static, Result<ShadowEventsResponse>>;

    fn shadow_events<'s: 'fut, 'fut>(
        &'s self,
        request: Request<ShadowEventsRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::ShadowEventsStream>>> {
        async move {
            let ShadowEventsRequest { selector } = request.into_inner();
            let shadow = self
                .core
                .select_entity::<Shadow>(selector.unwrap_or_default())?;
            let stream = shadow
                .subscribe()
                .map(|res| match res {
                    Ok(event) => Ok(ShadowEventsResponse { event: Some(event) }),
                    // TODO
                    Err(err) => Err(Status::internal(format!("failed to receive event: {err}"))),
                })
                .boxed();
            Ok(Response::new(stream))
        }
        .boxed()
    }
}

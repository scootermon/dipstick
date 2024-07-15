use std::sync::Arc;

use dipstick_core::Core;
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::shadow::v1::{
    CreateShadowRequest, CreateShadowResponse, GetShadowRequest, GetShadowResponse,
    ShadowServiceServer, ShadowSignalEventsRequest, ShadowSignalEventsResponse, ShadowSpec,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::FutureExt;
use tonic::{Request, Response, Result};

pub use self::shadow::Shadow;

mod listeners;
mod shadow;

pub const PACKAGE: &str = "shadow.v1";

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
            let selector = selector.unwrap_or_default();
            let shadow = self.core.select_entity::<Shadow>(&selector)?;
            Ok(Response::new(GetShadowResponse {
                shadow: Some(shadow.to_proto()),
            }))
        }
        .boxed()
    }

    type ShadowSignalEventsStream = BoxStream<'static, Result<ShadowSignalEventsResponse>>;

    fn shadow_signal_events<'s: 'fut, 'fut>(
        &'s self,
        request: Request<ShadowSignalEventsRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::ShadowSignalEventsStream>>> {
        async move {
            let ShadowSignalEventsRequest { selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let _shadow = self.core.select_entity::<Shadow>(&selector)?;
            todo!()
        }
        .boxed()
    }
}

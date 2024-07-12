use dipstick_proto::shadow::v1::{
    CreateShadowRequest,
    CreateShadowResponse,
    ShadowSignalEventsRequest,
    ShadowSignalEventsResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::FutureExt;
use tonic::{Request, Response};

mod shadow;

pub struct ShadowService {}

impl dipstick_proto::shadow::v1::ShadowService for ShadowService {
    fn create_shadow<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateShadowRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<CreateShadowResponse>>> {
        async move { todo!() }.boxed()
    }

    type ShadowSignalEventsStream = BoxStream<'static, tonic::Result<ShadowSignalEventsResponse>>;

    fn shadow_signal_events<'s: 'fut, 'fut>(
        &'s self,
        request: Request<ShadowSignalEventsRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<Self::ShadowSignalEventsStream>>> {
        async move { todo!() }.boxed()
    }
}

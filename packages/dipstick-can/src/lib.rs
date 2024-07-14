use std::sync::Arc;

use dipstick_core::Core;
use dipstick_proto::can::v1::{
    BusSpec, CanService, CanServiceServer, CreateBusRequest, CreateBusResponse, GetBusRequest,
    GetBusResponse, ReceiveFramesRequest, ReceiveFramesResponse, SendFrameRequest,
    SendFrameResponse,
};
use dipstick_proto::core::v1::EntityMetaSpec;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tonic::{Request, Response, Result, Status};

pub use self::bus::Bus;

mod bus;

const PACKAGE: &str = "can.v1";

pub struct Can {
    core: Arc<Core>,
}

impl Can {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> CanServiceServer<Self> {
        CanServiceServer::from_arc(self)
    }

    pub async fn create_bus_impl(&self, meta: EntityMetaSpec, spec: BusSpec) -> Result<Arc<Bus>> {
        let (meta, reservation) = self.core.new_entity_meta::<Bus>(meta)?;
        let bus = Bus::new(meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&bus));
        Ok(bus)
    }
}

impl CanService for Can {
    fn create_bus<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateBusRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateBusResponse>>> {
        async move {
            let CreateBusRequest { meta, spec } = request.into_inner();
            let bus = self
                .create_bus_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateBusResponse {
                bus: Some(bus.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_bus<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetBusRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetBusResponse>>> {
        async move {
            let GetBusRequest { bus: selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let bus = self.core.select_entity::<Bus>(&selector)?;
            Ok(Response::new(GetBusResponse {
                bus: Some(bus.to_proto()),
            }))
        }
        .boxed()
    }

    fn send_frame<'s: 'fut, 'fut>(
        &'s self,
        request: Request<SendFrameRequest>,
    ) -> BoxFuture<'fut, Result<Response<SendFrameResponse>>> {
        async move {
            let SendFrameRequest {
                bus: selector,
                frame,
            } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let frame = frame.unwrap_or_default();
            let bus = self.core.select_entity::<Bus>(&selector)?;
            bus.send_frame(&frame).await?;
            Ok(Response::new(SendFrameResponse {}))
        }
        .boxed()
    }

    type ReceiveFramesStream = BoxStream<'static, Result<ReceiveFramesResponse>>;

    fn receive_frames<'s: 'fut, 'fut>(
        &'s self,
        request: Request<ReceiveFramesRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::ReceiveFramesStream>>> {
        async move {
            let ReceiveFramesRequest { bus: selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let bus = self.core.select_entity::<Bus>(&selector)?;

            let stream: Self::ReceiveFramesStream = bus
                .subscribe()
                .map(|res| {
                    match res {
                        Ok(frame) => Ok(ReceiveFramesResponse { frame: Some(frame) }),
                        // TODO
                        Err(err) => {
                            Err(Status::internal(format!("failed to receive frame: {err}")))
                        }
                    }
                })
                .boxed();
            Ok(Response::new(stream))
        }
        .boxed()
    }
}

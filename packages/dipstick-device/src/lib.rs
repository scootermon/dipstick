use std::sync::Arc;

use dipstick_core::Core;
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::device::v1::{
    CallActionRequest, CallActionResponse, CreateDeviceRequest, CreateDeviceResponse,
    DeviceEventsRequest, DeviceEventsResponse, DeviceServiceServer, DeviceSpec, GetDeviceRequest,
    GetDeviceResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tonic::{Request, Response, Result, Status};

pub use self::device::Device;

mod device;
mod devices;
mod sensor;

pub const PACKAGE: &str = "device.v1";

pub struct DeviceService {
    core: Arc<Core>,
}

impl DeviceService {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> DeviceServiceServer<Self> {
        DeviceServiceServer::from_arc(self)
    }

    pub async fn create_device_impl(
        &self,
        meta: EntityMetaSpec,
        spec: DeviceSpec,
    ) -> Result<Arc<Device>> {
        let (meta, reservation) = self.core.new_entity_meta::<Device>(meta)?;
        let device = Device::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&device));
        Ok(device)
    }
}

impl dipstick_proto::device::v1::DeviceService for DeviceService {
    fn create_device<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateDeviceRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateDeviceResponse>>> {
        async move {
            let CreateDeviceRequest { meta, spec } = request.into_inner();
            let device = self
                .create_device_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateDeviceResponse {
                device: Some(device.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_device<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetDeviceRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetDeviceResponse>>> {
        async move {
            let GetDeviceRequest { selector } = request.into_inner();
            let device = self
                .core
                .select_entity::<Device>(selector.unwrap_or_default())?;
            Ok(Response::new(GetDeviceResponse {
                device: Some(device.to_proto()),
            }))
        }
        .boxed()
    }

    fn call_action<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CallActionRequest>,
    ) -> BoxFuture<'fut, Result<Response<CallActionResponse>>> {
        async move {
            let CallActionRequest { selector, action } = request.into_inner();
            let device = self
                .core
                .select_entity::<Device>(selector.unwrap_or_default())?;
            device.call_action(&action).await?;
            Ok(Response::new(CallActionResponse {}))
        }
        .boxed()
    }

    type DeviceEventsStream = BoxStream<'static, Result<DeviceEventsResponse>>;

    fn device_events<'s: 'fut, 'fut>(
        &'s self,
        request: Request<DeviceEventsRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::DeviceEventsStream>>> {
        async move {
            let DeviceEventsRequest { selector } = request.into_inner();
            let device = self
                .core
                .select_entity::<Device>(selector.unwrap_or_default())?;
            let stream = device
                .subscribe()
                .map(|res| {
                    match res {
                        Ok(event) => Ok(DeviceEventsResponse { event: Some(event) }),
                        // TODO
                        Err(err) => {
                            Err(Status::internal(format!("failed to receive event: {err}")))
                        }
                    }
                })
                .boxed();
            Ok(Response::new(stream))
        }
        .boxed()
    }
}

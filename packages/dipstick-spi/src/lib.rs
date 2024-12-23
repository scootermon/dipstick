use std::sync::Arc;

use dipstick_core::{Core, Package, PackageKind};
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::spi::v1::{
    CreateDeviceRequest, CreateDeviceResponse, DeviceSpec, GetDeviceRequest, GetDeviceResponse,
    SpiServiceServer, TransferRequest, TransferResponse, TransfersRequest, TransfersResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tonic::{Request, Response, Result, Status};

pub use self::device::Device;

mod device;

pub struct SpiService {
    core: Arc<Core>,
}

impl SpiService {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> SpiServiceServer<Self> {
        SpiServiceServer::from_arc(self)
    }

    pub async fn create_device_impl(
        &self,
        meta: EntityMetaSpec,
        spec: DeviceSpec,
    ) -> Result<Arc<Device>> {
        let (meta, reservation) = self.core.new_entity_meta::<Device>(meta)?;
        let device = Device::new(meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&device));
        Ok(device)
    }
}

impl Package for SpiService {
    fn package_name(&self) -> &'static str {
        Self::PACKAGE_NAME
    }
}

impl PackageKind for SpiService {
    const PACKAGE_NAME: &'static str = "spi.v1";
}

impl dipstick_proto::spi::v1::SpiService for SpiService {
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
            let selector = selector.unwrap_or_default();
            let device = self.core.select_entity::<Device>(&selector)?;
            Ok(Response::new(GetDeviceResponse {
                device: Some(device.to_proto()),
            }))
        }
        .boxed()
    }

    fn transfer<'s: 'fut, 'fut>(
        &'s self,
        request: Request<TransferRequest>,
    ) -> BoxFuture<'fut, Result<Response<TransferResponse>>> {
        async move {
            let TransferRequest { selector, data } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let device = self.core.select_entity::<Device>(&selector)?;
            let rx_data = device.transfer(data).await?;
            Ok(Response::new(TransferResponse { data: rx_data }))
        }
        .boxed()
    }

    type TransfersStream = BoxStream<'static, Result<TransfersResponse>>;

    fn transfers<'s: 'fut, 'fut>(
        &'s self,
        request: Request<TransfersRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::TransfersStream>>> {
        async move {
            let TransfersRequest { selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let device = self.core.select_entity::<Device>(&selector)?;
            let stream: Self::TransfersStream = device
                .subscribe()
                .map(|res| {
                    match res {
                        Ok(v) => Ok(v),
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

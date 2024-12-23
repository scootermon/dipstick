use std::sync::Arc;

use dipstick_core::{Core, Package, PackageKind};
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::gpio::v1::{
    ChipSpec, CreateChipRequest, CreateChipResponse, GetChipRequest, GetChipResponse, GpioService,
    GpioServiceServer, Level, SetPinLevelRequest, SetPinLevelResponse, SubscribeChipRequest,
    SubscribeChipResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tonic::{Request, Response, Result, Status};

pub use self::chip::Chip;

mod chip;

pub struct Gpio {
    core: Arc<Core>,
}

impl Gpio {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> GpioServiceServer<Self> {
        GpioServiceServer::from_arc(self)
    }

    pub async fn create_chip_impl(
        &self,
        meta: EntityMetaSpec,
        spec: ChipSpec,
    ) -> Result<Arc<Chip>> {
        let (meta, reservation) = self.core.new_entity_meta::<Chip>(meta)?;
        let chip = Chip::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&chip));
        Ok(chip)
    }
}

impl Package for Gpio {
    fn package_name(&self) -> &'static str {
        Self::PACKAGE_NAME
    }
}

impl PackageKind for Gpio {
    const PACKAGE_NAME: &'static str = "gpio.v1";
}

impl GpioService for Gpio {
    fn create_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateChipRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateChipResponse>>> {
        async move {
            let CreateChipRequest { meta, spec } = request.into_inner();
            let chip = self
                .create_chip_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateChipResponse {
                chip: Some(chip.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetChipRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetChipResponse>>> {
        async move {
            let GetChipRequest { chip: selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let chip = self.core.select_entity::<Chip>(&selector)?;
            Ok(Response::new(GetChipResponse {
                chip: Some(chip.to_proto()),
            }))
        }
        .boxed()
    }

    type SubscribeChipStream = BoxStream<'static, Result<SubscribeChipResponse>>;

    fn subscribe_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<SubscribeChipRequest>,
    ) -> BoxFuture<'fut, Result<Response<Self::SubscribeChipStream>>> {
        async move {
            let SubscribeChipRequest { chip: selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let chip = self.core.select_entity::<Chip>(&selector)?;
            let stream: Self::SubscribeChipStream = chip
                .subscribe()
                .map(|res| match res {
                    Ok(event) => Ok(event),
                    Err(err) => Err(Status::internal(err.to_string())),
                })
                .boxed();
            Ok(Response::new(stream))
        }
        .boxed()
    }

    fn set_pin_level<'s: 'fut, 'fut>(
        &'s self,
        request: Request<SetPinLevelRequest>,
    ) -> BoxFuture<'fut, Result<Response<SetPinLevelResponse>>> {
        async move {
            let SetPinLevelRequest {
                chip,
                pin_id,
                logical,
            } = request.into_inner();
            let logical = Level::try_from(logical).unwrap_or_default();
            let chip = self.core.select_entity::<Chip>(&chip.unwrap_or_default())?;
            chip.set_pin_level(pin_id, logical).await?;
            Ok(Response::new(SetPinLevelResponse {}))
        }
        .boxed()
    }
}

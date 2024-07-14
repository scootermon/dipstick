use std::sync::Arc;

use dipstick_core::Core;
use dipstick_proto::gpio::v1::{
    CreateChipRequest, CreateChipResponse, GetChipRequest, GetChipResponse, GpioService,
    GpioServiceServer, SubscribeChipRequest, SubscribeChipResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use tonic::{Code, Request, Response, Result, Status};

pub use self::chip::Chip;

mod chip;

const PACKAGE: &str = "gpio.v1";

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
}

impl GpioService for Gpio {
    fn create_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateChipRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateChipResponse>>> {
        async move {
            let CreateChipRequest { meta, spec } = request.into_inner();
            let meta = meta.unwrap_or_default();
            let spec = spec.unwrap_or_default();

            let (meta, reservation) = self.core.new_entity_meta::<Chip>(meta)?;
            let chip = Chip::new(meta, spec).await.map_err(|err| {
                Status::new(Code::Unknown, format!("failed to create chip: {err}"))
            })?;
            self.core
                .registry()
                .add_entity(reservation, Arc::clone(&chip));
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
            let chip = self.core.registry().select::<Chip>(&selector)?;
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
            let chip = self.core.registry().select::<Chip>(&selector)?;
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
}

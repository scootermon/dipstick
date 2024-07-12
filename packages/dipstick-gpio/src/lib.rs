use std::sync::Arc;

use dipstick_core::{EntitySelector, IdContext, QualifiedId, Registry};
use dipstick_proto::gpio::v1::{
    CreateChipRequest,
    CreateChipResponse,
    GetChipRequest,
    GetChipResponse,
    GpioService,
    GpioServiceServer,
};
use futures::future::BoxFuture;
use futures::FutureExt;
use tonic::{Code, Request, Response, Status};

pub use self::chip::Chip;

mod chip;

const PACKAGE: &str = "gpio.v1";

pub struct Gpio {
    registry: Arc<Registry>,
}

impl Gpio {
    pub fn new(registry: Arc<Registry>) -> Arc<Self> {
        Arc::new(Self { registry })
    }

    pub fn into_server(self: Arc<Self>) -> GpioServiceServer<Self> {
        GpioServiceServer::from_arc(self)
    }
}

impl GpioService for Gpio {
    fn create_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateChipRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<CreateChipResponse>>> {
        async move {
            let CreateChipRequest { chip } = request.into_inner();
            let chip = chip.unwrap_or_default();
            let chip_meta = chip.meta.unwrap_or_default();

            // TODO: force package and domain to match
            let id = QualifiedId::parse_with_context(
                &chip_meta.id,
                &IdContext::full_ref(PACKAGE, chip::DOMAIN),
            )
            .map_err(|err| Status::invalid_argument(format!("invalid chip id: {err}")))?;

            let reservation = self.registry.reserve_id(id.to_static()).ok_or_else(|| {
                Status::already_exists(format!("chip with id '{id}' already exists"))
            })?;

            let chip = Chip::new(id.to_static(), chip.spec.unwrap_or_default())
                .await
                .map_err(|err| {
                    Status::new(Code::Unknown, format!("failed to create chip: {err}"))
                })?;
            let chip = Arc::new(chip);
            self.registry
                .add_entity_with_reservation(reservation, Arc::clone(&chip));
            Ok(Response::new(CreateChipResponse {
                chip: Some(chip.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetChipRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<GetChipResponse>>> {
        async move {
            let GetChipRequest { chip: selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let selector =
                EntitySelector::from_proto(&selector, &IdContext::full_ref(PACKAGE, chip::DOMAIN))
                    .map_err(|err| {
                        Status::invalid_argument(format!("invalid chip selector: {err}"))
                    })?;
            let chip = self
                .registry
                .get_by_selector::<Chip>(&selector)
                .ok_or_else(|| Status::not_found("chip not found"))?;
            Ok(Response::new(GetChipResponse {
                chip: Some(chip.to_proto()),
            }))
        }
        .boxed()
    }
}

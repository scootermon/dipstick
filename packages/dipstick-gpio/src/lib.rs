use std::sync::Arc;

use dipstick_core::{EntitySelector, QualifiedId, Registry};
use dipstick_proto::gpio::v1::{
    CreateChipRequest, CreateChipResponse, GetChipRequest, GetChipResponse, GpioService,
    GpioServiceServer, SubscribeChipRequest, SubscribeChipResponse,
};
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
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
            let CreateChipRequest { meta, spec } = request.into_inner();
            let meta = meta.unwrap_or_default();
            let spec = spec.unwrap_or_default();

            let (id, reservation) = if meta.id.is_empty() {
                (None, None)
            } else {
                // TODO: force package and domain to match
                let id = QualifiedId::parse_with_context(&meta.id, &chip::ID_CONTEXT)
                    .map_err(|err| Status::invalid_argument(format!("invalid chip id: {err}")))?;
                let reservation = self.registry.reserve_id(id.to_static()).ok_or_else(|| {
                    Status::already_exists(format!("chip with id '{id}' already exists"))
                })?;
                (Some(id.to_static()), Some(reservation))
            };

            let chip = Chip::new(id, spec).await.map_err(|err| {
                Status::new(Code::Unknown, format!("failed to create chip: {err}"))
            })?;
            let chip = Arc::new(chip);
            if let Some(reservation) = reservation {
                self.registry
                    .add_entity_with_reservation(reservation, Arc::clone(&chip));
            } else {
                // LOGIC: no id
                self.registry.add_entity(Arc::clone(&chip)).unwrap();
            }
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
            let chip = request.into_inner().select_chip(&self.registry)?;
            Ok(Response::new(GetChipResponse {
                chip: Some(chip.to_proto()),
            }))
        }
        .boxed()
    }

    type SubscribeChipStream = BoxStream<'static, tonic::Result<SubscribeChipResponse>>;

    fn subscribe_chip<'s: 'fut, 'fut>(
        &'s self,
        request: Request<SubscribeChipRequest>,
    ) -> BoxFuture<'fut, tonic::Result<Response<Self::SubscribeChipStream>>> {
        async move {
            let chip = request.into_inner().select_chip(&self.registry)?;
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

trait ChipSelector {
    fn raw_selector(&self) -> Option<&dipstick_proto::core::v1::EntitySelector>;

    fn chip_selector(&self) -> tonic::Result<EntitySelector> {
        let selector = self
            .raw_selector()
            .ok_or_else(|| Status::invalid_argument("chip selector missing"))?;
        let selector = EntitySelector::from_proto(selector, &chip::ID_CONTEXT)
            .map_err(|err| Status::invalid_argument(format!("invalid chip selector: {err}")))?;
        Ok(selector)
    }

    fn select_chip(&self, registry: &Registry) -> tonic::Result<Arc<Chip>> {
        let selector = self.chip_selector()?;
        let chip = registry
            .get_by_selector::<Chip>(&selector)
            .ok_or_else(|| Status::not_found("chip not found"))?;
        Ok(chip)
    }
}

impl ChipSelector for GetChipRequest {
    fn raw_selector(&self) -> Option<&dipstick_proto::core::v1::EntitySelector> {
        self.chip.as_ref()
    }
}

impl ChipSelector for SubscribeChipRequest {
    fn raw_selector(&self) -> Option<&dipstick_proto::core::v1::EntitySelector> {
        self.chip.as_ref()
    }
}

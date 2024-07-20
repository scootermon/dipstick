use std::sync::Arc;

use dipstick_can::Can;
use dipstick_core::Core;
use dipstick_gpio::Gpio;
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::stack::v1::{
    CreateStackRequest, CreateStackResponse, GetStackRequest, GetStackResponse, StackServiceServer,
    StackSpec,
};
use dipstick_shadow::ShadowService;
use dipstick_spi::Spi;
use dipstick_xcp::XcpService;
use futures::future::BoxFuture;
use futures::FutureExt;
use tonic::{Request, Response, Result};

use self::packages::Packages;
pub use self::stack::Stack;

mod packages;
mod stack;

pub const PACKAGE: &str = "stack.v1";

pub struct StackService {
    core: Arc<Core>,
    packages: Packages,
}

impl StackService {
    pub fn new(
        core: Arc<Core>,
        can: Arc<Can>,
        gpio: Arc<Gpio>,
        spi: Arc<Spi>,
        xcp: Arc<XcpService>,
        shadow: Arc<ShadowService>,
    ) -> Arc<Self> {
        let packages = Packages {
            can,
            gpio,
            spi,
            xcp,
            shadow,
        };
        Arc::new(Self { core, packages })
    }

    pub fn into_server(self: Arc<Self>) -> StackServiceServer<Self> {
        StackServiceServer::from_arc(self)
    }

    pub async fn create_stack_impl(
        &self,
        meta: EntityMetaSpec,
        spec: StackSpec,
    ) -> Result<Arc<Stack>> {
        let (meta, reservation) = self.core.new_entity_meta::<Stack>(meta)?;
        let stack = Stack::new(&self.packages, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&stack));
        Ok(stack)
    }
}

impl dipstick_proto::stack::v1::StackService for StackService {
    fn create_stack<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateStackRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateStackResponse>>> {
        async move {
            let CreateStackRequest { meta, spec } = request.into_inner();
            let stack = self
                .create_stack_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateStackResponse {
                stack: Some(stack.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_stack<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetStackRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetStackResponse>>> {
        async move {
            let GetStackRequest { selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let stack = self.core.select_entity::<Stack>(&selector)?;
            Ok(Response::new(GetStackResponse {
                stack: Some(stack.to_proto()),
            }))
        }
        .boxed()
    }
}

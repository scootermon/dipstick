use std::sync::Arc;

use dipstick_core::{Core, Package, PackageKind};
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::stack::v1::{
    CreateStackRequest, CreateStackResponse, GetStackRequest, GetStackResponse, StackServiceServer,
    StackSpec,
};
use futures::future::BoxFuture;
use futures::FutureExt;
use tonic::{Request, Response, Result};

pub use self::stack::Stack;

mod stack;

pub struct StackService {
    core: Arc<Core>,
}

impl StackService {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
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
        let stack = Stack::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&stack));
        Ok(stack)
    }
}

impl Package for StackService {
    fn package_name(&self) -> &'static str {
        Self::PACKAGE_NAME
    }
}

impl PackageKind for StackService {
    const PACKAGE_NAME: &'static str = "stack.v1";
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

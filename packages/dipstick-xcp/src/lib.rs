use std::sync::Arc;

use dipstick_core::Core;
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::xcp::v1::{
    A2lSpec, CreateA2lRequest, CreateA2lResponse, CreateSessionRequest, CreateSessionResponse,
    GetA2lRequest, GetA2lResponse, GetSessionRequest, GetSessionResponse, XcpServiceServer,
};
use futures::future::BoxFuture;
use futures::FutureExt;
use tonic::{Request, Response, Result};

pub use self::a2l::A2l;
pub use self::session::Session;

mod a2l;
mod session;
mod transport;

pub const PACKAGE: &str = "xcp.v1";

pub struct XcpService {
    core: Arc<Core>,
}

impl XcpService {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> XcpServiceServer<Self> {
        XcpServiceServer::from_arc(self)
    }

    pub async fn create_a2l_impl(&self, meta: EntityMetaSpec, spec: A2lSpec) -> Result<Arc<A2l>> {
        let (meta, reservation) = self.core.new_entity_meta::<A2l>(meta)?;
        let a2l = A2l::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&a2l));
        Ok(a2l)
    }

    pub async fn create_session_impl(&self) -> Result<Arc<Session>> {
        todo!()
    }
}

impl dipstick_proto::xcp::v1::XcpService for XcpService {
    fn create_a2l<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateA2lRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateA2lResponse>>> {
        async move {
            let CreateA2lRequest { meta, spec } = request.into_inner();
            let a2l = self
                .create_a2l_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateA2lResponse {
                a2l: Some(a2l.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_a2l<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetA2lRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetA2lResponse>>> {
        async move {
            let GetA2lRequest { selector } = request.into_inner();
            let selector = selector.unwrap_or_default();
            let a2l = self.core.select_entity::<A2l>(&selector)?;
            Ok(Response::new(GetA2lResponse {
                a2l: Some(a2l.to_proto()),
            }))
        }
        .boxed()
    }

    fn create_session<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateSessionRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateSessionResponse>>> {
        async move { todo!() }.boxed()
    }

    fn get_session<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetSessionRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetSessionResponse>>> {
        async move { todo!() }.boxed()
    }
}

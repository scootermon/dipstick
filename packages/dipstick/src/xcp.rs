use std::sync::Arc;

use dipstick_proto::xcp::v1::{
    CommandRequest,
    CommandResponse,
    ConnectSessionRequest,
    ConnectSessionResponse,
    CreateRequest,
    CreateResponse,
    DestroyRequest,
    DestroyResponse,
    XcpService,
    XcpServiceServer,
};
use tonic::{Request, Response, Status};

use self::session::Session;
use crate::core::registry::Registry;

mod codec;
mod session;
mod transport;

pub struct Xcp {
    sessions: Registry<Session>,
}

impl Xcp {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: Registry::new(),
        })
    }

    pub fn into_server(self: Arc<Self>) -> XcpServiceServer<Self> {
        XcpServiceServer::from_arc(self)
    }
}

#[async_trait::async_trait]
impl XcpService for Xcp {
    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> tonic::Result<Response<CreateResponse>> {
        let CreateRequest {
            disconnect_response_optional,
            transport,
        } = request.into_inner();
        todo!()
    }

    async fn destroy(
        &self,
        request: Request<DestroyRequest>,
    ) -> tonic::Result<Response<DestroyResponse>> {
        let DestroyRequest { session_id } = request.into_inner();
        todo!()
    }

    async fn connect_session(
        &self,
        request: Request<ConnectSessionRequest>,
    ) -> tonic::Result<Response<ConnectSessionResponse>> {
        let ConnectSessionRequest { session_id, mode } = request.into_inner();
        todo!()
    }

    async fn command(
        &self,
        request: Request<CommandRequest>,
    ) -> tonic::Result<Response<CommandResponse>> {
        let CommandRequest {
            session_id,
            command,
        } = request.into_inner();
        let command = command.ok_or_else(|| Status::invalid_argument("'command' unspecified"))?;
        let session = self.sessions.get(session_id)?;
        let response = session.raw_command(command).await?;
        Ok(Response::new(CommandResponse {
            response: Some(response),
        }))
    }
}

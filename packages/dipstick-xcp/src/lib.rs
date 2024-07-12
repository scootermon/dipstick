use std::sync::Arc;

use dipstick_proto::xcp::v1::{
    CommandRequest,
    CommandResponse,
    ConnectSessionRequest,
    ConnectSessionResponse,
    CreateSessionRequest,
    CreateSessionResponse,
    DestroySessionRequest,
    DestroySessionResponse,
    PeriodicShortUploadRequest,
    PeriodicShortUploadResponse,
    XcpService,
    XcpServiceServer,
};
use futures::stream::BoxStream;
use tonic::{Request, Response, Status};

use self::session::Session;
use crate::core::registry::Registry;

mod codec;
mod periodic_short_upload;
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

    // TODO: someone needs to call this
    pub fn cleanup(&self) {
        self.sessions.for_each(|_, session| session.cleanup());
    }

    pub async fn shutdown(&self) {
        self.sessions
            .for_each_async(|_, session| async move { session.shutdown().await });
    }

    pub fn into_server(self: Arc<Self>) -> XcpServiceServer<Self> {
        XcpServiceServer::from_arc(self)
    }
}

#[async_trait::async_trait]
impl XcpService for Xcp {
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> tonic::Result<Response<CreateSessionResponse>> {
        let CreateSessionRequest {
            disconnect_response_optional,
            transport,
        } = request.into_inner();
        todo!()
    }

    async fn destroy_session(
        &self,
        request: Request<DestroySessionRequest>,
    ) -> tonic::Result<Response<DestroySessionResponse>> {
        let DestroySessionRequest { session_id } = request.into_inner();
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

    type PeriodicShortUploadStream = periodic_short_upload::Stream;

    async fn periodic_short_upload(
        &self,
        request: Request<PeriodicShortUploadRequest>,
    ) -> tonic::Result<Response<Self::PeriodicShortUploadStream>> {
        let PeriodicShortUploadRequest {
            session_id,
            command,
            interval,
            mut ignore_interval_if_exists,
        } = request.into_inner();
        let command = command.ok_or_else(|| Status::invalid_argument("'command' unspecified"))?;
        // if no interval is specified, ignore_interval_if_exists is always true
        ignore_interval_if_exists |= interval.is_none();
        let interval = interval
            .unwrap_or_default()
            .try_into()
            .map_err(|_| Status::invalid_argument("'interval' invalid"))?;
        let session = self.sessions.get(session_id)?;
        let stream = session.periodic_short_upload(command, interval, ignore_interval_if_exists)?;
        Ok(Response::new(stream))
    }
}

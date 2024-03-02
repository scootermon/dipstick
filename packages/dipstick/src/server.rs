use dipstick_proto::server::{
    LogConfigRequest,
    LogConfigResponse,
    LogSubscribeRequest,
    LogSubscribeResponse,
    ServerService,
    ServerServiceServer,
    VersionRequest,
    VersionResponse,
};
use futures::stream::BoxStream;
use tonic::{Request, Response};

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_server(self) -> ServerServiceServer<Self> {
        ServerServiceServer::new(self)
    }
}

#[async_trait::async_trait]
impl ServerService for Server {
    async fn version(
        &self,
        request: Request<VersionRequest>,
    ) -> tonic::Result<Response<VersionResponse>> {
        Ok(Response::new(VersionResponse {
            version: crate::consts::VERSION.to_owned(),
        }))
    }

    async fn log_config(
        &self,
        request: Request<LogConfigRequest>,
    ) -> tonic::Result<Response<LogConfigResponse>> {
        todo!()
    }

    type LogSubscribeStream = BoxStream<'static, tonic::Result<LogSubscribeResponse>>;

    async fn log_subscribe(
        &self,
        request: Request<LogSubscribeRequest>,
    ) -> tonic::Result<Response<Self::LogSubscribeStream>> {
        todo!()
    }
}

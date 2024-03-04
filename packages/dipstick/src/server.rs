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
use futures::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response};

pub mod logging;

pub struct Server {
    log_handle: logging::LoggingHandle,
}

impl Server {
    pub fn new(log_handle: logging::LoggingHandle) -> Self {
        Self { log_handle }
    }

    pub fn into_server(self) -> ServerServiceServer<Self> {
        ServerServiceServer::new(self)
    }
}

#[async_trait::async_trait]
impl ServerService for Server {
    async fn version(
        &self,
        _request: Request<VersionRequest>,
    ) -> tonic::Result<Response<VersionResponse>> {
        Ok(Response::new(VersionResponse {
            version: crate::consts::VERSION.to_owned(),
        }))
    }

    async fn log_config(
        &self,
        request: Request<LogConfigRequest>,
    ) -> tonic::Result<Response<LogConfigResponse>> {
        let LogConfigRequest { config } = request.into_inner();
        if let Some(config) = config {
            // update config
            self.log_handle
                .set_log_config(config)
                .map_err(|err| tonic::Status::internal(err.to_string()))?;
        }

        let Some(config) = self.log_handle.log_config() else {
            return Err(tonic::Status::internal("subscriber dropped"));
        };
        Ok(Response::new(LogConfigResponse {
            config: Some(config),
        }))
    }

    type LogSubscribeStream = BoxStream<'static, tonic::Result<LogSubscribeResponse>>;

    async fn log_subscribe(
        &self,
        _request: Request<LogSubscribeRequest>,
    ) -> tonic::Result<Response<Self::LogSubscribeStream>> {
        let stream = BroadcastStream::new(self.log_handle.logs_tx.subscribe())
            .filter_map(|item| {
                std::future::ready(match item {
                    Ok(event) => Some(Ok(LogSubscribeResponse { event: Some(event) })),
                    _ => None,
                })
            })
            .boxed();

        Ok(Response::new(stream))
    }
}

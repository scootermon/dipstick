use std::sync::Arc;

use dipstick_proto::core::v1::{
    CoreService,
    CoreServiceServer,
    LogConfigRequest,
    LogConfigResponse,
    LogSubscribeRequest,
    LogSubscribeResponse,
    ShutdownRequest,
    ShutdownResponse,
    VersionRequest,
    VersionResponse,
};
use futures::stream::BoxStream;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response};

pub mod history;
pub mod logging;
pub mod registry;

pub struct Core {
    log_handle: logging::LoggingHandle,
    shutdown_tx: mpsc::Sender<()>,
}

impl Core {
    pub fn new(log_handle: logging::LoggingHandle, shutdown_tx: mpsc::Sender<()>) -> Arc<Self> {
        Arc::new(Self {
            log_handle,
            shutdown_tx,
        })
    }

    pub fn into_server(self: Arc<Self>) -> CoreServiceServer<Self> {
        CoreServiceServer::from_arc(self)
    }
}

#[async_trait::async_trait]
impl CoreService for Core {
    async fn shutdown(
        &self,
        _request: Request<ShutdownRequest>,
    ) -> tonic::Result<Response<ShutdownResponse>> {
        tracing::info!("received shutdown request");
        let _ = self.shutdown_tx.send(()).await;
        Ok(Response::new(ShutdownResponse {}))
    }

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

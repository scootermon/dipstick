use std::sync::Arc;

use dipstick_proto::can::v1::{
    CanService,
    CanServiceServer,
    CreateSocketRequest,
    CreateSocketResponse,
    DriverInfo,
    ListDriversRequest,
    ListDriversResponse,
    SendFrameRequest,
    SendFrameResponse,
};
use tonic::{Request, Response, Status};

pub use self::socket::Socket;
use crate::core::registry::Registry;

mod socket;

pub struct Can {
    sockets: Registry<Socket>,
}

impl Can {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sockets: Registry::new(),
        })
    }

    pub fn into_server(self: Arc<Self>) -> CanServiceServer<Self> {
        CanServiceServer::from_arc(self)
    }
}

#[async_trait::async_trait]
impl CanService for Can {
    async fn list_drivers(
        &self,
        request: Request<ListDriversRequest>,
    ) -> tonic::Result<Response<ListDriversResponse>> {
        let ListDriversRequest { id: _ } = request.into_inner();
        let drivers = vec![DriverInfo {
            driver_id: "null".to_owned(),
            name: "null".to_owned(),
            description: "Dummy driver for testing".to_owned(),
            version: "0.1.0".to_owned(),
        }];
        Ok(Response::new(ListDriversResponse { drivers }))
    }

    async fn create_socket(
        &self,
        request: tonic::Request<CreateSocketRequest>,
    ) -> tonic::Result<Response<CreateSocketResponse>> {
        let CreateSocketRequest {} = request.into_inner();
        let socket = self.sockets.add_with_factory(Socket::new_null);
        Ok(Response::new(CreateSocketResponse {
            socket: Some(socket.info.clone()),
        }))
    }

    async fn send_frame(
        &self,
        request: tonic::Request<SendFrameRequest>,
    ) -> tonic::Result<Response<SendFrameResponse>> {
        let SendFrameRequest { socket_id, frame } = request.into_inner();
        let Some(frame) = frame else {
            return Err(Status::invalid_argument("frame must be provided"));
        };
        let socket = self.sockets.get(socket_id)?;
        socket.send_frame(frame).await?;
        Ok(Response::new(SendFrameResponse {}))
    }
}

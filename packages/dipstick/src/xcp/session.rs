use std::time::SystemTime;

use bytes::BytesMut;
use dipstick_proto::xcp::v1::{ByteOrder, ConnectResponse, CtoCommand, CtoResponse, Frame};
use tokio::sync::broadcast;
use tonic::Status;

use super::transport::Transport;
use crate::xcp::codec;

pub struct Session {
    connect_response: ConnectResponse,
    transport: Transport,
    frame_tx: broadcast::Sender<Frame>,
}

impl Session {
    fn byte_order(&self) -> ByteOrder {
        self.connect_response
            .comm_mode_basic
            .as_ref()
            .map_or(ByteOrder::Unspecified, |basic| basic.byte_order())
    }

    fn is_be(&self) -> bool {
        self.byte_order() == ByteOrder::Big
    }

    pub async fn raw_command(&self, mut command: CtoCommand) -> tonic::Result<CtoResponse> {
        let Some(payload) = &command.cto_command_content else {
            return Err(Status::invalid_argument("'command' is unspecified"));
        };
        let mut buf = BytesMut::new();
        codec::encode_command(payload, self.is_be(), &mut buf)?;
        self.transport.send(buf.freeze()).await?;
        command.timestamp = Some(SystemTime::now().into());
        let _ = self.frame_tx.send(
            Frame::builder()
                .direction_tx()
                .content_command(command)
                .build(),
        );

        todo!() // TODO: wait for response
    }
}

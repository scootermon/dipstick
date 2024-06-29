use std::sync::Arc;

use bytes::Bytes;
use dipstick_proto::can::v1::Frame as CanFrame;
use dipstick_proto::xcp::v1::CanTransport;

pub enum Transport {
    Can {
        socket: Arc<crate::can::Socket>,
        config: CanTransport,
    },
}

impl Transport {
    pub async fn send(&self, payload: Bytes) -> tonic::Result<()> {
        match self {
            Self::Can { socket, config } => {
                let frame = CanFrame::builder()
                    .id(config.server_id.unwrap_or_default())
                    .data(payload)
                    .build();
                socket.send_frame(frame).await
            }
        }
    }
}

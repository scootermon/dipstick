use std::time::SystemTime;

use dipstick_proto::can::v1::{Frame, SocketInfo};
use dipstick_proto::core::v1::Direction;
use dipstick_proto::wkt::Timestamp;
use tokio::sync::broadcast;

pub struct Socket {
    pub info: SocketInfo,
    tx: broadcast::Sender<Frame>,
}

impl Socket {
    pub fn new_null(id: u64) -> Self {
        let (tx, _) = tokio::sync::broadcast::channel(32); // TODO
        let info = SocketInfo {
            socket_id: id,
            created_at: Some(Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            driver_id: "null".to_owned(),
        }; // TODO
        Self { info, tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Frame> {
        self.tx.subscribe()
    }

    pub async fn send_frame(&self, mut frame: Frame) -> tonic::Result<()> {
        // TODO: actually send
        tracing::trace!(?frame);

        frame.set_direction(Direction::Transmit);
        frame.received_at = Some(SystemTime::now().into());
        let _ = self.tx.send(frame);
        Ok(())
    }
}

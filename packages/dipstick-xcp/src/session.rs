use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use bytes::BytesMut;
use dipstick_proto::xcp::v1::{
    ByteOrder,
    ConnectResponse,
    CtoCommand,
    CtoResponse,
    Frame,
    ShortUploadCommand,
};
use tokio::sync::broadcast;
use tonic::Status;

use super::periodic_short_upload;
use super::transport::Transport;
use crate::xcp::codec;

pub struct Session {
    connect_response: ConnectResponse,
    transport: Transport,
    frame_tx: broadcast::Sender<Frame>,
    periodic_short_uploads:
        std::sync::Mutex<BTreeMap<ShortUploadCommand, periodic_short_upload::Worker>>,
}

impl Session {
    pub fn cleanup(&self) {
        {
            let mut periodic_short_uploads = self
                .periodic_short_uploads
                .lock()
                .unwrap_or_else(|err| err.into_inner());
            periodic_short_uploads.retain(|_, worker| worker.is_running());
        }
    }

    pub async fn shutdown(&self) {
        {
            let mut periodic_short_upload = self
                .periodic_short_uploads
                .lock()
                .unwrap_or_else(|err| err.into_inner());
            for (_, worker) in periodic_short_upload.iter_mut() {
                worker.shutdown().await;
            }
            periodic_short_upload.clear();
        }
    }

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

    pub fn periodic_short_upload(
        self: &Arc<Self>,
        command: ShortUploadCommand,
        interval: Duration,
        ignore_interval_if_exists: bool,
    ) -> tonic::Result<periodic_short_upload::Stream> {
        let mut periodic_short_uploads = self.periodic_short_uploads.lock().unwrap();
        if let Some(worker) = periodic_short_uploads.get(&command) {
            if !ignore_interval_if_exists && worker.interval() != interval {
                return Err(Status::already_exists(
                    "periodic short upload already exists with a different interval",
                ));
            }
            if let Some(stream) = worker.subscribe() {
                // worker is running, return the existing stream
                return Ok(stream);
            }
        }

        // no worker running, spawn a new one
        let (worker, stream) =
            periodic_short_upload::Worker::new(Arc::clone(self), command.clone(), interval);
        periodic_short_uploads.insert(command, worker);
        Ok(stream)
    }
}

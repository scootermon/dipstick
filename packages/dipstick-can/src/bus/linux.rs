use std::time::SystemTime;

use dipstick_proto::can::v1::{Frame, LinuxBusSpec};
use futures::StreamExt;
use socketcan::tokio::{CanFdSocket, CanSocket};
use socketcan::SocketOptions;
use tokio::sync::broadcast;
use tokio::task::{block_in_place, JoinHandle};
use tokio_util::sync::{CancellationToken, DropGuard};
use tonic::{Result, Status};

pub mod conv;

pub struct Bus {
    socket: CanSocket,
    _drop_guard: DropGuard,
    _reader_task: JoinHandle<()>,
}

impl Bus {
    // TODO: shutdown

    pub async fn new(spec: &mut LinuxBusSpec, tx: broadcast::Sender<Frame>) -> Result<Self> {
        let cancel_token = CancellationToken::new();

        let (socket, reader_task) = block_in_place(|| -> anyhow::Result<_> {
            set_up_interface_blocking(spec)?;
            let socket = CanSocket::open(&spec.device)?;
            socket.set_loopback(false)?;
            let reader_task = spawn_reader_task_blocking(&spec.device, cancel_token.clone(), tx)?;
            Ok((socket, reader_task))
        })
        .map_err(|err| Status::internal(format!("failed to set up can interface: {err}")))?;

        Ok(Self {
            socket,
            _drop_guard: cancel_token.drop_guard(),
            _reader_task: reader_task,
        })
    }

    pub async fn send(&self, frame: &Frame) -> Result<()> {
        let frame = conv::to_linux_frame(frame)?;
        let fut = self
            .socket
            .write_frame(frame)
            .map_err(|err| Status::internal(format!("failed to duplicate socket: {err}")))?;
        match fut.await {
            Ok(()) => Ok(()),
            Err(err) => {
                tracing::error!(
                    ?frame,
                    err = &err as &dyn std::error::Error,
                    "failed to send frame"
                );
                Err(Status::internal(format!("failed to send frame: {err}")))
            }
        }
    }
}

fn set_up_interface_blocking(spec: &mut LinuxBusSpec) -> anyhow::Result<()> {
    let can_if = socketcan::CanInterface::open(&spec.device)?;
    let current_bitrate = can_if.bit_rate()?;
    match spec.bitrate {
        Some(bitrate) if Some(bitrate) != current_bitrate => {
            tracing::debug!(current_bitrate, bitrate, "changing bitrate to match spec");
            can_if.bring_down()?;
            can_if.set_bitrate(bitrate, None)?;
        }
        Some(_) => {
            // bitrate already matches
        }
        None if current_bitrate.is_some() => {
            spec.bitrate = current_bitrate;
        }
        None => {
            anyhow::bail!("bitrate not specified and none configured on interface");
        }
    }
    tracing::debug!("bringing up interface");
    can_if.bring_up()?;
    Ok(())
}

fn spawn_reader_task_blocking(
    ifname: &str,
    cancel_token: CancellationToken,
    tx: broadcast::Sender<Frame>,
) -> Result<JoinHandle<()>> {
    let socket = CanFdSocket::open(ifname)?;
    socket.set_loopback(false)?;
    socket.set_error_filter_accept_all()?;
    socket.set_filter_accept_all()?;
    Ok(ReaderTask {
        cancel_token,
        socket,
        tx,
    }
    .spawn())
}

struct ReaderTask {
    cancel_token: CancellationToken,
    socket: CanFdSocket,
    tx: broadcast::Sender<Frame>,
}

impl ReaderTask {
    fn spawn(self) -> JoinHandle<()> {
        // TODO: span
        tokio::spawn(self.run())
    }

    async fn run(mut self) {
        loop {
            let item = tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    break;
                }
                item = self.socket.next() => item,
            };
            match item {
                Some(Ok(frame)) => {
                    // TODO: see: https://github.com/socketcan-rs/socketcan-rs/issues/22
                    let timestamp = SystemTime::now();

                    let frame = conv::from_linux_frame(&frame, timestamp);
                    let _ = self.tx.send(frame);
                }
                Some(Err(err)) => {
                    // TODO: inject error frame I guess?
                    tracing::error!(err = &err as &dyn std::error::Error, "failed to read frame");
                }
                None => {
                    // should never stop
                    unreachable!()
                }
            }
        }
    }
}

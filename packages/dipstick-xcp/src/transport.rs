use std::sync::Arc;

use bytes::Bytes;
use dipstick_can::Bus;
use dipstick_core::{Core, DependencyHandle, Entity, EntityMeta};
use dipstick_proto::xcp::v1::{
    CanTransportSpec, CtoErrRespData, CtoResp, CtoRespData, CtoRespPacket, CtoRespPid,
    XcpTransportSpec,
};
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Result, Status};

pub enum Transport {
    Can(CanTransport),
}

impl Transport {
    pub async fn new(core: &Core, meta: &EntityMeta, spec: &mut XcpTransportSpec) -> Result<Self> {
        match spec {
            XcpTransportSpec::Can(can) => CanTransport::new(core, meta, can).await.map(Self::Can),
        }
    }

    pub fn subscribe(&self) -> BroadcastStream<CtoRespPacket> {
        match self {
            Self::Can(transport) => BroadcastStream::new(transport.packet_tx.subscribe()),
        }
    }

    pub async fn send(&self, packet_data: Bytes) -> Result<()> {
        tracing::trace!(?packet_data, "sending packet");
        match self {
            Self::Can(transport) => transport.send(packet_data).await,
        }
    }
}

pub async fn get_response(
    packet_stream: &mut BroadcastStream<CtoRespPacket>,
    mut parse_fn: impl FnMut(Bytes) -> anyhow::Result<CtoResp>,
) -> Result<CtoResp> {
    loop {
        match packet_stream.next().await {
            Some(Ok(packet)) => match packet.pid() {
                CtoRespPid::Ok => {
                    match parse_fn(packet.data) {
                        Ok(mut resp) => {
                            resp.timestamp = packet.timestamp;
                            return Ok(resp);
                        }
                        Err(err) => {
                            // TODO error event?
                            tracing::error!(error = &*err, "failed to parse response");
                        }
                    }
                }
                CtoRespPid::Err => {
                    return Ok(CtoResp {
                        timestamp: packet.timestamp,
                        cto_resp_data: Some(CtoRespData::Error(CtoErrRespData {
                            error: packet.error,
                            data: packet.data,
                        })),
                    });
                }
                _ => {}
            },
            Some(Err(BroadcastStreamRecvError::Lagged(missed))) => {
                // TODO error event?
                tracing::error!(missed, "packet stream lagged");
            }
            None => return Err(Status::cancelled("transport closed")),
        }
    }
}

pub struct CanTransport {
    bus: Arc<Bus>,
    _dependency_handle: DependencyHandle,
    server_id: dipstick_proto::can::v1::Id,
    packet_tx: broadcast::Sender<CtoRespPacket>,
    _handle: tokio::task::JoinHandle<()>,
}

impl CanTransport {
    pub async fn new(core: &Core, meta: &EntityMeta, spec: &mut CanTransportSpec) -> Result<Self> {
        let bus = spec.bus.clone().unwrap_or_default();
        let bus = core.select_entity::<dipstick_can::Bus>(&bus)?;
        let dependency_handle = meta.add_dependency(bus.entity_meta());

        let server_id = spec
            .server_id
            .ok_or_else(|| Status::invalid_argument("missing server id"))?;
        let client_id = spec
            .client_id
            .ok_or_else(|| Status::invalid_argument("missing client id"))?;

        let (packet_tx, _) = broadcast::channel(32); // TODO
        let handle = tokio::spawn({
            let mut stream = bus.subscribe();
            let packet_tx = packet_tx.clone();
            async move {
                loop {
                    let mut can_frame = match stream.next().await {
                        Some(Ok(frame)) => {
                            if !frame.id.as_ref().is_some_and(|id| *id == client_id) {
                                // not the can id we care about
                                continue;
                            }
                            frame
                        }
                        Some(Err(BroadcastStreamRecvError::Lagged(missed))) => {
                            // TODO: error event?
                            tracing::error!(missed, "xcp can bus reader lagged");
                            continue;
                        }
                        None => break,
                    };
                    let mut packet = crate::protocol::decoder::packet(&mut can_frame.data).unwrap(); // TODO
                    packet.timestamp = can_frame.timestamp;

                    tracing::trace!(?packet, "received packet");
                    let _ = packet_tx.send(packet);
                }
            }
        });

        Ok(Self {
            bus,
            _dependency_handle: dependency_handle,
            server_id,
            packet_tx,
            _handle: handle,
        })
    }

    async fn send(&self, packet_data: Bytes) -> Result<()> {
        // TODO: pad packet if required
        let can_frame = dipstick_proto::can::v1::Frame {
            id: Some(self.server_id),
            data: packet_data,
            ..Default::default()
        };
        self.bus.send_frame(&can_frame).await
    }
}

use std::sync::Arc;

use dipstick_core::{Entity, EntityKind, EntityMeta};
use dipstick_proto::can::v1::{BusEntity, BusSpec, BusSpecVariant, BusStatus, Frame};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Result, Status};

mod linux;

pub struct Bus {
    meta: EntityMeta,
    spec: BusSpec,
    linux: linux::Bus,
    tx: broadcast::Sender<Frame>,
}

impl Bus {
    pub async fn new(meta: EntityMeta, mut spec: BusSpec) -> Result<Arc<Self>> {
        let (tx, _) = broadcast::channel(128); // TODO
        let linux = match &mut spec.bus_spec_variant {
            Some(BusSpecVariant::Linux(linux)) => linux::Bus::new(linux, tx.clone()).await?,
            None => return Err(Status::invalid_argument("missing bus spec variant")),
        };
        Ok(Arc::new(Self {
            meta,
            spec,
            linux,
            tx,
        }))
    }

    pub fn to_proto(&self) -> BusEntity {
        let status = BusStatus {};
        BusEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec.clone()),
            status: Some(status),
        }
    }

    pub async fn send_frame(&self, frame: &Frame) -> Result<()> {
        self.linux.send(frame).await
    }

    pub fn subscribe(&self) -> BroadcastStream<Frame> {
        BroadcastStream::new(self.tx.subscribe())
    }
}

impl Entity for Bus {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Bus {
    const PACKAGE: &'static str = crate::PACKAGE;
    const KIND: &'static str = "Bus";
}

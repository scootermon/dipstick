use std::sync::Arc;

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
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
    pub async fn new(core: &Core, meta: EntityMeta, mut spec: BusSpec) -> Result<Arc<Self>> {
        let (tx, _) = broadcast::channel(128); // TODO
        let linux = match &mut spec.bus_spec_variant {
            Some(BusSpecVariant::Linux(linux)) => linux::Bus::new(core, linux, tx.clone()).await?,
            None => return Err(Status::invalid_argument("missing bus spec variant")),
        };
        Ok(Arc::new(Self {
            meta,
            spec,
            linux,
            tx,
        }))
    }

    pub fn spec(&self) -> BusSpec {
        self.spec.clone()
    }

    pub fn status(&self) -> BusStatus {
        BusStatus {}
    }

    pub fn to_proto(&self) -> BusEntity {
        BusEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub async fn send_frame(&self, frame: &Frame) -> Result<()> {
        tracing::trace!(?frame, "sending frame");
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
    type Package = crate::Can;
    const KIND: &'static str = "Bus";
}

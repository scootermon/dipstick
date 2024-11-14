use std::sync::Arc;

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::can::v1::{BusEntity, BusSpec, BusSpecVariant, BusStatus, Frame};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Result, Status};

#[cfg(target_os = "linux")]
mod linux;

pub struct Bus {
    meta: EntityMeta,
    spec: BusSpec,
    variant: Variant,
    tx: broadcast::Sender<Frame>,
}

impl Bus {
    pub async fn new(core: &Core, meta: EntityMeta, mut spec: BusSpec) -> Result<Arc<Self>> {
        let (tx, _) = broadcast::channel(128); // TODO
        let variant_spec = spec
            .bus_spec_variant
            .as_mut()
            .ok_or_else(|| Status::invalid_argument("missing bus spec variant"))?;
        let variant = Variant::new(core, tx.clone(), variant_spec).await?;
        Ok(Arc::new(Self {
            meta,
            spec,
            variant,
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
        self.variant.send(frame).await
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

enum Variant {
    #[cfg(target_os = "linux")]
    Linux(linux::Bus),
}

impl Variant {
    async fn new(
        core: &Core,
        tx: broadcast::Sender<Frame>,
        variant_spec: &mut BusSpecVariant,
    ) -> Result<Self> {
        match variant_spec {
            #[cfg(target_os = "linux")]
            BusSpecVariant::Linux(linux) => {
                let bus = linux::Bus::new(core, linux, tx.clone()).await?;
                Ok(Self::Linux(bus))
            }
            #[cfg(not(target_os = "linux"))]
            BusSpecVariant::Linux(_) => {
                let _ = (core, tx);
                Err(Status::invalid_argument("linux bus support not available"))
            }
        }
    }

    async fn send(&self, frame: &Frame) -> Result<()> {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux(bus) => bus.send(frame).await,
            #[cfg(not(target_os = "linux"))]
            _ => {
                let _ = frame;
                unreachable!()
            }
        }
    }
}

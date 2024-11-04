use std::sync::Arc;
use std::time::SystemTime;

use bytes::Bytes;
use dipstick_core::{Entity, EntityKind, EntityMeta};
use dipstick_proto::spi::v1::{
    DeviceEntity, DeviceSpec, DeviceSpecVariant, DeviceStatus, TransfersResponse,
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Result, Status};

#[cfg(target_os = "linux")]
mod linux;

pub struct Device {
    meta: EntityMeta,
    spec: DeviceSpec,
    variant: Variant,
    tx: broadcast::Sender<TransfersResponse>,
}

impl Device {
    pub async fn new(meta: EntityMeta, mut spec: DeviceSpec) -> Result<Arc<Self>> {
        // Take it so we can use mutable references to both the spec and the inner
        // variant spec. We'll put it back at the end.
        let mut variant_spec = spec
            .device_spec_variant
            .take()
            .ok_or_else(|| Status::invalid_argument("missing device spec variant"))?;
        let variant = Variant::new(&mut spec, &mut variant_spec).await?;
        spec.device_spec_variant = Some(variant_spec);
        let (tx, _rx) = broadcast::channel(128); // TODO
        tracing::info!("created device");
        Ok(Arc::new(Self {
            meta,
            spec,
            variant,
            tx,
        }))
    }

    pub fn spec(&self) -> DeviceSpec {
        self.spec.clone()
    }

    pub fn status(&self) -> DeviceStatus {
        DeviceStatus {}
    }

    pub fn to_proto(&self) -> DeviceEntity {
        DeviceEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub fn subscribe(&self) -> BroadcastStream<TransfersResponse> {
        BroadcastStream::new(self.tx.subscribe())
    }

    pub async fn transfer(&self, data: Bytes) -> Result<Bytes> {
        tracing::trace!("running transfer");
        let start = SystemTime::now();
        let rx = match &self.variant {
            #[cfg(target_os = "linux")]
            Variant::Linux(device) => {
                let rx = device.transfer(&data)?;
                Bytes::from(rx)
            }
            #[cfg(not(target_os = "linux"))]
            _ => {
                // unreachable!
                let _ = data;
                Bytes::new()
            }
        };

        let duration = SystemTime::now()
            .duration_since(start)
            .ok()
            .and_then(|dur| dur.try_into().ok());
        let _ = self.tx.send(TransfersResponse {
            timestamp: Some(start.into()),
            duration,
            rx: rx.clone(),
            tx: data.clone(),
        });
        Ok(rx)
    }
}

impl Entity for Device {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Device {
    type Package = crate::SpiService;
    const KIND: &'static str = "Device";
}

enum Variant {
    #[cfg(target_os = "linux")]
    Linux(linux::Device),
}

impl Variant {
    async fn new(spec: &mut DeviceSpec, variant_spec: &mut DeviceSpecVariant) -> Result<Self> {
        match variant_spec {
            #[cfg(target_os = "linux")]
            DeviceSpecVariant::Linux(linux) => {
                let device = linux::Device::new(spec, linux)?;
                Ok(Self::Linux(device))
            }
            #[cfg(not(target_os = "linux"))]
            DeviceSpecVariant::Linux(_) => {
                let _ = spec;
                Err(Status::unimplemented("linux device support not available"))
            }
        }
    }
}

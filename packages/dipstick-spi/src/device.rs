use std::sync::Arc;

use bytes::Bytes;
use dipstick_core::{Entity, EntityKind, EntityMeta};
use dipstick_proto::spi::v1::{DeviceEntity, DeviceSpec, DeviceSpecVariant, DeviceStatus};
use tonic::{Result, Status};

#[cfg(target_os = "linux")]
mod linux;

pub struct Device {
    meta: EntityMeta,
    spec: DeviceSpec,
    variant: Variant,
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
        tracing::info!("created device");
        Ok(Arc::new(Self {
            meta,
            spec,
            variant,
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

    pub async fn transfer(&self, data: Bytes) -> Result<Bytes> {
        tracing::trace!("running transfer");
        match &self.variant {
            #[cfg(target_os = "linux")]
            Variant::Linux(device) => {
                let rx = device.transfer(&data)?;
                Ok(Bytes::from(rx))
            }
        }
    }
}

impl Entity for Device {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Device {
    const PACKAGE: &'static str = crate::PACKAGE;
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
                Err(Status::unimplemented("linux device support not available"))
            }
        }
    }
}

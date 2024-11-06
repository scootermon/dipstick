use bytes::{Buf, BufMut, BytesMut};
use dipstick_core::{Core, Dep};
use dipstick_proto::core::v1::EntitySelector;
use dipstick_proto::device::v1::Ina2xxTransportSpec;
use tonic::Result;

use super::register::Register;
use crate::Device;

pub struct Transport {
    inner: Inner,
}

impl Transport {
    pub fn new(core: &Core, device: &Device, spec: Ina2xxTransportSpec) -> Result<Self> {
        match spec {
            Ina2xxTransportSpec::SpiDevice(selector) => {
                let transport = SpiTransport::new(core, device, selector)?;
                Ok(Self {
                    inner: Inner::Spi(transport),
                })
            }
        }
    }

    pub fn spec(&self) -> Ina2xxTransportSpec {
        match &self.inner {
            Inner::Spi(transport) => Ina2xxTransportSpec::SpiDevice(transport.spec()),
        }
    }

    pub async fn read_register<R: Register>(&self) -> Result<R> {
        match &self.inner {
            Inner::Spi(spi) => spi.read_register().await,
        }
    }

    pub async fn write_register<R: Register>(&self, reg: R) -> Result<()> {
        match &self.inner {
            Inner::Spi(spi) => spi.write_register(reg).await,
        }
    }
}

enum Inner {
    Spi(SpiTransport),
}

struct SpiTransport {
    spec: EntitySelector,
    device: Dep<dipstick_spi::Device>,
}

impl SpiTransport {
    fn new(core: &Core, device: &Device, selector: EntitySelector) -> Result<Self> {
        let device = core.select_entity_dep(device, &selector)?;
        // TODO: ensure max_speed < 10e6
        // TODO: ensure spi mode == 1
        Ok(Self {
            spec: selector,
            device,
        })
    }

    fn spec(&self) -> EntitySelector {
        self.spec.clone()
    }

    async fn read_register<R: Register>(&self) -> Result<R> {
        let mut tx_data = BytesMut::with_capacity(1 + R::len());
        tx_data.put_u8((R::addr() << 2) | 0x01);
        tx_data.put_bytes(0, R::len());
        let mut rx_data = self.device.transfer(tx_data.freeze()).await?;
        tracing::trace!(addr = R::addr(), len = R::len(), data = ?rx_data, "read register");
        rx_data.advance(1);
        Ok(R::decode(&mut rx_data))
    }

    async fn write_register<R: Register>(&self, reg: R) -> Result<()> {
        let mut tx_data = BytesMut::with_capacity(1 + R::len());
        tx_data.put_u8(R::addr() << 2);
        reg.encode(&mut tx_data);
        self.device.transfer(tx_data.freeze()).await?;
        tracing::trace!(addr = R::addr(), len = R::len(), "wrote register");
        Ok(())
    }
}

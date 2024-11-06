use std::fmt::Display;

use bytes::{Buf, BufMut, BytesMut};
use dipstick_core::{Core, Dep};
use dipstick_proto::core::v1::EntitySelector;
use dipstick_proto::device::v1::Ina2xxTransportSpec;
use tonic::{Result, Status};

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
        let device = core.select_entity_dep::<dipstick_spi::Device>(device, &selector)?;
        if let Some(reason) = SpiDeviceInvalidReason::validate(&device.spec()) {
            return Err(Status::invalid_argument(format!(
                "selected SPI device ({selector}) is invalid: {reason}"
            )));
        }

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

const SPI_MAX_SPEED: u32 = 10_000_000; // 10 MHz

enum SpiDeviceInvalidReason {
    Mode,
    MaxSpeed,
}

impl SpiDeviceInvalidReason {
    fn validate(spec: &dipstick_proto::spi::v1::DeviceSpec) -> Option<Self> {
        use dipstick_proto::spi::v1::SpiMode;
        if !matches!(spec.mode(), SpiMode::SpiMode1 | SpiMode::Unspecified) {
            return Some(Self::Mode);
        }
        if spec.max_speed_hz() > SPI_MAX_SPEED {
            return Some(Self::MaxSpeed);
        }
        None
    }
}

impl Display for SpiDeviceInvalidReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mode => write!(f, "'mode' must be SPI_MODE_1"),
            Self::MaxSpeed => write!(f, "'max_speed_hz' must be <= 10 MHz"),
        }
    }
}

use std::sync::Arc;

use dipstick_can::Can;
use dipstick_device::DeviceService;
use dipstick_gpio::Gpio;
use dipstick_shadow::ShadowService;
use dipstick_spi::Spi;
use dipstick_xcp::XcpService;

pub struct Packages {
    pub can: Arc<Can>,
    pub device: Arc<DeviceService>,
    pub gpio: Arc<Gpio>,
    pub shadow: Arc<ShadowService>,
    pub spi: Arc<Spi>,
    pub xcp: Arc<XcpService>,
}

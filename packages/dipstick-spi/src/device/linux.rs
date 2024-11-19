use std::io;
use std::os::fd::AsRawFd;

use dipstick_proto::spi::v1::{DeviceSpec, LinuxDeviceSpec, SpiMode};
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use tokio::task::block_in_place;

pub struct Device(Spidev);

impl Device {
    pub fn new(spec: &mut DeviceSpec, linux_spec: &mut LinuxDeviceSpec) -> io::Result<Self> {
        block_in_place(|| Self::new_blocking(spec, linux_spec))
    }

    fn new_blocking(spec: &mut DeviceSpec, linux_spec: &mut LinuxDeviceSpec) -> io::Result<Self> {
        let path = format!(
            "/dev/spidev{}.{}",
            linux_spec.bus(),
            linux_spec.chipselect()
        );
        let mut device = Spidev::open(&path)?;
        apply_spec_to_device_blocking(&mut device, spec)?;
        Ok(Self(device))
    }

    pub fn transfer(&self, data: &[u8]) -> io::Result<Vec<u8>> {
        block_in_place(|| self.transfer_blocking(data))
    }

    fn transfer_blocking(&self, data: &[u8]) -> io::Result<Vec<u8>> {
        let mut rx_buf = vec![0; data.len()];
        let mut transfer = SpidevTransfer::read_write(data, &mut rx_buf);
        self.0.transfer(&mut transfer)?;
        Ok(rx_buf)
    }
}

fn apply_spec_to_device_blocking(device: &mut Spidev, spec: &mut DeviceSpec) -> io::Result<()> {
    write_device_config_blocking(device, spec)?;
    read_device_config_blocking(device, spec)?;
    Ok(())
}

fn write_device_config_blocking(device: &mut Spidev, spec: &DeviceSpec) -> io::Result<()> {
    let mut options = SpidevOptions::new();
    if let Some(mode) = to_linux_mode(spec.mode()) {
        options.mode(mode);
    }
    if let Some(value) = spec.bits_per_word {
        let value = value.try_into().map_err(|_err| {
            io::Error::new(io::ErrorKind::InvalidInput, "bits_per_word out of range")
        })?;
        options.bits_per_word(value);
    }
    if let Some(value) = spec.max_speed_hz {
        options.max_speed_hz(value);
    }
    if let Some(value) = spec.lsb_first {
        options.lsb_first(value);
    }
    device.configure(&options)?;
    tracing::trace!("wrote device configuration");
    Ok(())
}

fn read_device_config_blocking(device: &mut Spidev, spec: &mut DeviceSpec) -> io::Result<()> {
    let value = spidev::spidevioctl::get_mode_u32(device.as_raw_fd())?;
    spec.set_mode(from_linux_mode(SpiModeFlags::from_bits_retain(value)));
    let value = spidev::spidevioctl::get_bits_per_word(device.as_raw_fd())?;
    spec.bits_per_word = Some(value.into());
    let value = spidev::spidevioctl::get_max_speed_hz(device.as_raw_fd())?;
    spec.max_speed_hz = Some(value);
    let value = spidev::spidevioctl::get_lsb_first(device.as_raw_fd())?;
    spec.lsb_first = Some(value != 0);
    tracing::trace!("read device configuration");
    Ok(())
}

const fn to_linux_mode(mode: SpiMode) -> Option<SpiModeFlags> {
    match mode {
        SpiMode::Unspecified => None,
        SpiMode::SpiMode0 => Some(SpiModeFlags::SPI_MODE_0),
        SpiMode::SpiMode1 => Some(SpiModeFlags::SPI_MODE_1),
        SpiMode::SpiMode2 => Some(SpiModeFlags::SPI_MODE_2),
        SpiMode::SpiMode3 => Some(SpiModeFlags::SPI_MODE_3),
    }
}

const fn from_linux_mode(mode: SpiModeFlags) -> SpiMode {
    let cpol = mode.contains(SpiModeFlags::SPI_CPOL);
    let cpha = mode.contains(SpiModeFlags::SPI_CPHA);
    SpiMode::from_cpol_cpha(cpol, cpha)
}

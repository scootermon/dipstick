use bytes::{Buf, BufMut, Bytes, BytesMut};

pub trait Register: Sized {
    fn addr() -> u8;
    fn len() -> usize;
    fn decode(buf: &mut Bytes) -> Self;

    fn encode(&self, buf: &mut BytesMut);
}

#[derive(Debug, Default)]
pub struct Config {
    // 1-bit
    pub rst: bool,
    // 1-bit
    pub rst_acc: bool,
    // 8-bit
    pub conversion_delay: u8,
    // 1-bit
    pub temp_comp: bool,
    // 1-bit
    pub adc_range: AdcRange,
    // 4-bit padding
}

#[derive(Clone, Copy, Debug, Default)]
pub enum AdcRange {
    #[default]
    Range0,
    Range1,
}

impl Register for Config {
    fn addr() -> u8 {
        0x00
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        let adc_range = match (value >> 4) & 0x1 {
            0 => AdcRange::Range0,
            1 => AdcRange::Range1,
            _ => unreachable!(),
        };
        let temp_comp = ((value >> 5) & 0x1) == 1;
        let conversion_delay = ((value >> 6) & 0xFF) as u8;
        let rst_acc = ((value >> 14) & 0x1) == 1;
        let rst = ((value >> 15) & 0x1) == 1;
        Self {
            adc_range,
            temp_comp,
            conversion_delay,
            rst_acc,
            rst,
        }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let adc_range_bit = match self.adc_range {
            AdcRange::Range0 => 0,
            AdcRange::Range1 => 1,
        };
        let value = (self.rst as u16) << 15
            | (self.rst_acc as u16) << 14
            | (self.conversion_delay as u16) << 6
            | (self.temp_comp as u16) << 5
            | adc_range_bit << 4;
        buf.put_u16(value);
    }
}

#[derive(Debug, Default)]
pub struct AdcConfig {
    // 4-bit
    pub mode: u8,
    // 3-bit
    pub vbus_ct: ConversionTime,
    // 3-bit
    pub vsh_ct: ConversionTime,
    // 3-bit
    pub vt_ct: ConversionTime,
    // 3-bit
    pub avg: AvgSamples,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum ConversionTime {
    #[default]
    Us50,
    Us84,
    Us150,
    Us280,
    Us540,
    Us1052,
    Us2074,
    Us4120,
}

impl ConversionTime {
    const fn reg_value(self) -> u8 {
        match self {
            Self::Us50 => 0,
            Self::Us84 => 1,
            Self::Us150 => 2,
            Self::Us280 => 3,
            Self::Us540 => 4,
            Self::Us1052 => 5,
            Self::Us2074 => 6,
            Self::Us4120 => 7,
        }
    }

    const fn from_reg_value(value: u8) -> Self {
        match value {
            0 => Self::Us50,
            1 => Self::Us84,
            2 => Self::Us150,
            3 => Self::Us280,
            4 => Self::Us540,
            5 => Self::Us1052,
            6 => Self::Us2074,
            7 => Self::Us4120,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum AvgSamples {
    #[default]
    N1,
    N4,
    N16,
    N64,
    N128,
    N256,
    N512,
    N1024,
}

impl AvgSamples {
    const fn reg_value(self) -> u8 {
        match self {
            Self::N1 => 0,
            Self::N4 => 1,
            Self::N16 => 2,
            Self::N64 => 3,
            Self::N128 => 4,
            Self::N256 => 5,
            Self::N512 => 6,
            Self::N1024 => 7,
        }
    }

    const fn from_reg_value(value: u8) -> Self {
        match value {
            0 => Self::N1,
            1 => Self::N4,
            2 => Self::N16,
            3 => Self::N64,
            4 => Self::N128,
            5 => Self::N256,
            6 => Self::N512,
            7 => Self::N1024,
            _ => unreachable!(),
        }
    }
}

impl Register for AdcConfig {
    fn addr() -> u8 {
        0x01
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        let mode = ((value >> 12) & 0xF) as u8;
        let vbus_ct = ConversionTime::from_reg_value(((value >> 9) & 0x7) as u8);
        let vsh_ct = ConversionTime::from_reg_value(((value >> 6) & 0x7) as u8);
        let vt_ct = ConversionTime::from_reg_value(((value >> 3) & 0x7) as u8);
        let avg = AvgSamples::from_reg_value((value & 0x7) as u8);
        Self {
            mode,
            vbus_ct,
            vsh_ct,
            vt_ct,
            avg,
        }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = (self.mode as u16 & 0xF) << 12
            | (self.vbus_ct.reg_value() as u16) << 9
            | (self.vsh_ct.reg_value() as u16) << 6
            | (self.vt_ct.reg_value() as u16) << 3
            | (self.avg.reg_value() as u16);
        buf.put_u16(value);
    }
}

#[derive(Debug, Default)]
pub struct ShuntCal {
    // 1-bit padding
    // 15-bit
    pub shunt_cal: u16,
}

impl Register for ShuntCal {
    fn addr() -> u8 {
        0x02
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        let shunt_cal = value & 0x7FFF;
        Self { shunt_cal }
    }

    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(self.shunt_cal & 0x7FFF);
    }
}

#[derive(Debug, Default)]
pub struct ShuntTempco {
    // 2-bit padding
    // 14-bit
    pub temp_co: u16,
}

impl Register for ShuntTempco {
    fn addr() -> u8 {
        0x03
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        let temp_co = value & 0x3FFF;
        Self { temp_co }
    }

    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(self.temp_co & 0x3FFF);
    }
}

#[derive(Debug, Default)]
pub struct VShunt3 {
    // 20-bit
    pub vshunt: i32,
    // 4-bit padding
}

impl Register for VShunt3 {
    fn addr() -> u8 {
        0x04
    }

    fn len() -> usize {
        3
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = decode_u24(buf);
        let vshunt = decode_signed_u32(value >> 4, 20);
        Self { vshunt }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = encode_signed_u32(self.vshunt, 20) << 4;
        encode_u24(buf, value);
    }
}

#[derive(Debug, Default)]
pub struct VBus3 {
    // 20-bit
    pub vbus: i32,
    // 4-bit padding
}

impl Register for VBus3 {
    fn addr() -> u8 {
        0x05
    }

    fn len() -> usize {
        3
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = decode_u24(buf);
        let vbus = decode_signed_u32(value >> 4, 20);
        Self { vbus }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = encode_signed_u32(self.vbus, 20) << 4;
        encode_u24(buf, value);
    }
}

#[derive(Debug, Default)]
pub struct DieTemp16 {
    // 16-bit
    pub die_temp: i16,
}

impl Register for DieTemp16 {
    fn addr() -> u8 {
        0x06
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        let die_temp = decode_signed_u16(value, 16);
        Self { die_temp }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = encode_signed_u16(self.die_temp, 16);
        buf.put_u16(value);
    }
}

#[derive(Debug, Default)]
pub struct Current3 {
    // 20-bit
    pub current: i32,
    // 4-bit padding
}

impl Register for Current3 {
    fn addr() -> u8 {
        0x07
    }

    fn len() -> usize {
        3
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = decode_u24(buf);
        let current = decode_signed_u32(value >> 4, 20);
        Self { current }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = encode_signed_u32(self.current, 20) << 4;
        encode_u24(buf, value);
    }
}

#[derive(Debug, Default)]
pub struct Power3 {
    // 24-bit
    pub power: u32,
}

impl Register for Power3 {
    fn addr() -> u8 {
        0x08
    }

    fn len() -> usize {
        3
    }

    fn decode(buf: &mut Bytes) -> Self {
        let power = decode_u24(buf);
        Self { power }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = self.power;
        encode_u24(buf, value);
    }
}

#[derive(Debug, Default)]
pub struct Energy {
    // 40-bit
    pub energy: u64,
}

impl Register for Energy {
    fn addr() -> u8 {
        0x09
    }

    fn len() -> usize {
        5
    }

    fn decode(buf: &mut Bytes) -> Self {
        let energy = decode_u40(buf);
        Self { energy }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = self.energy;
        encode_u40(buf, value);
    }
}

#[derive(Debug, Default)]
pub struct Charge {
    // 40-bit
    pub charge: i64,
}

impl Register for Charge {
    fn addr() -> u8 {
        0x0A
    }

    fn len() -> usize {
        5
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = decode_u40(buf);
        let charge = decode_signed_u64(value, 40);
        Self { charge }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = encode_signed_u64(self.charge, 40);
        encode_u40(buf, value);
    }
}

#[derive(Debug, Default)]
pub struct DiagAlert {
    // 1-bit
    pub alert_latch: bool,
    // 1-bit
    pub conv_ready: bool,
    // 1-bit
    pub slow_alert: bool,
    // 1-bit
    pub alert_pol: bool,
    // 1-bit
    pub energy_overflow: bool,
    // 1-bit
    pub charge_overflow: bool,
    // 1-bit
    pub math_overflow: bool,
    // 1-bit reserved
    // 1-bit
    pub temp_over_limit: bool,
    // 1-bit
    pub shunt_over_limit: bool,
    // 1-bit
    pub shunt_under_limit: bool,
    // 1-bit
    pub bus_over_limit: bool,
    // 1-bit
    pub bus_under_limit: bool,
    // 1-bit
    pub power_over_limit: bool,
    // 1-bit
    pub conv_complete: bool,
    // 1-bit
    pub mem_stat: bool,
}

impl Register for DiagAlert {
    fn addr() -> u8 {
        0x0B
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        Self {
            alert_latch: (value >> 15 & 0x1) == 1,
            conv_ready: (value >> 14 & 0x1) == 1,
            slow_alert: (value >> 13 & 0x1) == 1,
            alert_pol: (value >> 12 & 0x1) == 1,
            energy_overflow: (value >> 11 & 0x1) == 1,
            charge_overflow: (value >> 10 & 0x1) == 1,
            math_overflow: (value >> 9 & 0x1) == 1,
            temp_over_limit: (value >> 7 & 0x1) == 1,
            shunt_over_limit: (value >> 6 & 0x1) == 1,
            shunt_under_limit: (value >> 5 & 0x1) == 1,
            bus_over_limit: (value >> 4 & 0x1) == 1,
            bus_under_limit: (value >> 3 & 0x1) == 1,
            power_over_limit: (value >> 2 & 0x1) == 1,
            conv_complete: (value >> 1 & 0x1) == 1,
            mem_stat: (value & 0x1) == 1,
        }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = (self.alert_latch as u16) << 15
            | (self.conv_ready as u16) << 14
            | (self.slow_alert as u16) << 13
            | (self.alert_pol as u16) << 12
            | (self.energy_overflow as u16) << 11
            | (self.charge_overflow as u16) << 10
            | (self.math_overflow as u16) << 9
            | (self.temp_over_limit as u16) << 7
            | (self.shunt_over_limit as u16) << 6
            | (self.shunt_under_limit as u16) << 5
            | (self.bus_over_limit as u16) << 4
            | (self.bus_under_limit as u16) << 3
            | (self.power_over_limit as u16) << 2
            | (self.conv_complete as u16) << 1
            | (self.mem_stat as u16);
        buf.put_u16(value);
    }
}

#[derive(Debug, Default)]
pub struct ManufacturerId {
    pub manf_id: u16,
}

impl Register for ManufacturerId {
    fn addr() -> u8 {
        0x3E
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        Self {
            manf_id: buf.get_u16(),
        }
    }

    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(self.manf_id);
    }
}

#[derive(Debug, Default)]
pub struct DeviceId {
    pub die_id: u16,
    pub rev_id: u8,
}

impl Register for DeviceId {
    fn addr() -> u8 {
        0x3F
    }

    fn len() -> usize {
        2
    }

    fn decode(buf: &mut Bytes) -> Self {
        let value = buf.get_u16();
        let die_id = (value >> 4) & 0xFFF;
        let rev_id = (value & 0xF) as u8;
        Self { die_id, rev_id }
    }

    fn encode(&self, buf: &mut BytesMut) {
        let value = (self.die_id & 0xFFF) << 4 | (self.rev_id & 0xF) as u16;
        buf.put_u16(value);
    }
}

#[inline]
fn decode_u24(buf: &mut Bytes) -> u32 {
    let mut data = [0; 4];
    buf.copy_to_slice(&mut data[1..]);
    u32::from_be_bytes(data)
}

#[inline]
fn encode_u24(buf: &mut BytesMut, value: u32) {
    let data = value.to_be_bytes();
    buf.put_slice(&data[1..]);
}

#[inline]
fn decode_u40(buf: &mut Bytes) -> u64 {
    let mut data = [0; 8];
    buf.copy_to_slice(&mut data[3..]);
    u64::from_be_bytes(data)
}

#[inline]
fn encode_u40(buf: &mut BytesMut, value: u64) {
    let data = value.to_be_bytes();
    buf.put_slice(&data[3..]);
}

#[inline]
fn decode_signed_u64(value: u64, bits: u8) -> i64 {
    if value > (1 << (bits - 1)) {
        (value as i64).wrapping_sub(1 << bits)
    } else {
        value as i64
    }
}

#[inline]
fn encode_signed_u64(value: i64, bits: u8) -> u64 {
    if value < 0 {
        (value + (1 << bits)) as u64
    } else {
        value as u64
    }
}

#[inline]
fn decode_signed_u32(value: u32, bits: u8) -> i32 {
    decode_signed_u64(value as _, bits) as _
}

#[inline]
fn encode_signed_u32(value: i32, bits: u8) -> u32 {
    encode_signed_u64(value as _, bits) as _
}

#[inline]
fn decode_signed_u16(value: u16, bits: u8) -> i16 {
    decode_signed_u64(value as _, bits) as _
}

#[inline]
fn encode_signed_u16(value: i16, bits: u8) -> u16 {
    encode_signed_u64(value as _, bits) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_complement_works() {
        assert_eq!(decode_signed_u32(0x7FFF, 16), 0x7FFF);
        assert_eq!(decode_signed_u32(0x8000, 16), -0x8000);
        assert_eq!(decode_signed_u32(0xFFFF, 16), -1);

        assert_eq!(encode_signed_u32(0x7FFF, 16), 0x7FFF);
        assert_eq!(encode_signed_u32(-0x8000, 16), 0x8000);
        assert_eq!(encode_signed_u32(-1, 16), 0xFFFF);
    }

    #[track_caller]
    fn assert_register_basics<R: Register + Default>() {
        let reg = R::default();
        let mut buf = BytesMut::new();
        reg.encode(&mut buf);
        assert_eq!(buf.len(), R::len());
        let _reg_roundtrip = R::decode(&mut buf.freeze());
    }

    #[test]
    fn registers_basic() {
        assert_register_basics::<Config>();
        assert_register_basics::<AdcConfig>();
        assert_register_basics::<ShuntCal>();
        assert_register_basics::<ShuntTempco>();
        assert_register_basics::<VShunt3>();
        assert_register_basics::<VBus3>();
        assert_register_basics::<DieTemp16>();
        assert_register_basics::<Current3>();
        assert_register_basics::<Power3>();
        assert_register_basics::<Energy>();
        assert_register_basics::<Charge>();
        assert_register_basics::<DiagAlert>();
        assert_register_basics::<ManufacturerId>();
        assert_register_basics::<DeviceId>();
    }
}

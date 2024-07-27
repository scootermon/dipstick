use tonic::Result;

#[derive(Clone, Copy, Debug)]
pub enum Model {
    Ina228,
    Ina229,
}

impl Model {
    pub fn from_registers(manufacturer_id: u16, dev_id: u16) -> Result<Self> {
        let manufacturer_id = manufacturer_id.to_be_bytes();
        if manufacturer_id != *b"TI" {
            return Err(tonic::Status::internal(format!(
                "manufacturer id doesn't match 'TI': {manufacturer_id:x?}"
            )));
        }
        match dev_id {
            0x228 => Ok(Self::Ina228),
            0x229 => Ok(Self::Ina229),
            _ => Err(tonic::Status::internal(format!(
                "unknown device id: {dev_id:#x}"
            ))),
        }
    }

    pub const fn calibration(self) -> &'static Calibration {
        match self {
            Self::Ina228 | Self::Ina229 => &Calibration::INA228_INA229,
            #[allow(unreachable_patterns)]
            _ => &Calibration::OTHER,
        }
    }
}

pub struct Calibration {
    pub vbus_lsb: f32,
    pub vshunt_lsb_range0: f32,
    pub vshunt_lsb_range1: f32,
    pub shunt_cal_scale: f32,
    pub current_lsb_scale_factor: i8,
    pub die_temp_lsb: f32,
    pub power_coeff: f32,
    pub _energy_coeff: f32,
}

impl Calibration {
    const INA228_INA229: Self = Self {
        vbus_lsb: 0.0001953125,
        vshunt_lsb_range0: 0.0003125,
        vshunt_lsb_range1: 0.000078125,
        shunt_cal_scale: 13107.2 * 1000000.0,
        current_lsb_scale_factor: -19,
        die_temp_lsb: 0.0078125,
        power_coeff: 3.2,
        _energy_coeff: 16.0 * 3.2,
    };
    const OTHER: Self = Self {
        vbus_lsb: 0.003_125,
        vshunt_lsb_range0: 0.0050000,
        vshunt_lsb_range1: 0.001_25,
        shunt_cal_scale: 819.2 * 1000000.0,
        current_lsb_scale_factor: -15,
        die_temp_lsb: 0.125,
        power_coeff: 0.2,
        _energy_coeff: 0.0, // N/A
    };
}

use dipstick_proto::device::v1::Ina2xxSpec;
use tonic::{Result, Status};

use super::register::{AdcRange, AvgSamples, ConversionTime};

#[derive(Debug)]
pub struct Spec {
    pub shunt_resistance_ohm: f32,
    pub max_current_a: f32,
    pub shunt_tempco_ppm_c: u16,
    pub adc_range: AdcRange,
    pub bus_voltage_conversion_time: ConversionTime,
    pub shunt_voltage_conversion_time: ConversionTime,
    pub temperature_conversion_time: ConversionTime,
    pub avg_samples: AvgSamples,
}

impl Spec {
    pub fn from_proto(spec: Ina2xxSpec) -> Result<Self> {
        let shunt_tempco_ppm_c = spec
            .temperature_coefficient()
            .try_into()
            .map_err(|_err| Status::invalid_argument("invalid temperature coefficient"))?;
        let adc_range = match spec.adc_range() {
            0 => AdcRange::Range0,
            1 => AdcRange::Range1,
            _ => return Err(Status::invalid_argument("invalid adc range")),
        };
        let bus_voltage_conversion_time = match spec.bus_voltage_conversion_time {
            Some(v) => v.try_into()?,
            None => ConversionTime::Us4120,
        };
        let shunt_voltage_conversion_time = match spec.shunt_voltage_conversion_time {
            Some(v) => v.try_into()?,
            None => ConversionTime::Us4120,
        };
        let temperature_conversion_time = match spec.temperature_conversion_time {
            Some(v) => v.try_into()?,
            None => ConversionTime::Us4120,
        };
        let avg_samples = match spec.adc_averaging {
            Some(v) => v.try_into()?,
            None => AvgSamples::N128,
        };
        Ok(Self {
            shunt_resistance_ohm: spec.shunt_resistance(),
            max_current_a: spec.max_current(),
            shunt_tempco_ppm_c,
            adc_range,
            bus_voltage_conversion_time,
            shunt_voltage_conversion_time,
            temperature_conversion_time,
            avg_samples,
        })
    }

    pub fn to_proto(&self) -> Ina2xxSpec {
        let adc_range = match self.adc_range {
            AdcRange::Range0 => 0,
            AdcRange::Range1 => 1,
        };
        Ina2xxSpec {
            shunt_resistance: Some(self.shunt_resistance_ohm),
            max_current: Some(self.max_current_a),
            adc_range: Some(adc_range),
            bus_voltage_conversion_time: Some(self.bus_voltage_conversion_time.into()),
            shunt_voltage_conversion_time: Some(self.shunt_voltage_conversion_time.into()),
            temperature_conversion_time: Some(self.temperature_conversion_time.into()),
            adc_averaging: Some(self.avg_samples.into()),
            temperature_coefficient: Some(self.shunt_tempco_ppm_c.into()),
            ina2xx_transport_spec: None,
        }
    }
}

impl TryFrom<dipstick_proto::wkt::Duration> for ConversionTime {
    type Error = Status;

    fn try_from(value: dipstick_proto::wkt::Duration) -> Result<Self> {
        let us = value.seconds * 1_000_000 + i64::from(value.nanos / 1_000);
        match us {
            50 => Ok(Self::Us50),
            84 => Ok(Self::Us84),
            150 => Ok(Self::Us150),
            280 => Ok(Self::Us280),
            540 => Ok(Self::Us540),
            1052 => Ok(Self::Us1052),
            2074 => Ok(Self::Us2074),
            4120 => Ok(Self::Us4120),
            _ => Err(Status::invalid_argument("invalid conversion time")),
        }
    }
}

impl From<ConversionTime> for dipstick_proto::wkt::Duration {
    fn from(value: ConversionTime) -> Self {
        let us = match value {
            ConversionTime::Us50 => 50,
            ConversionTime::Us84 => 84,
            ConversionTime::Us150 => 150,
            ConversionTime::Us280 => 280,
            ConversionTime::Us540 => 540,
            ConversionTime::Us1052 => 1052,
            ConversionTime::Us2074 => 2074,
            ConversionTime::Us4120 => 4120,
        };
        dipstick_proto::wkt::Duration {
            seconds: 0,
            nanos: us * 1_000,
        }
    }
}

impl TryFrom<u32> for AvgSamples {
    type Error = Status;

    fn try_from(value: u32) -> Result<Self> {
        match value {
            1 => Ok(Self::N1),
            4 => Ok(Self::N4),
            16 => Ok(Self::N16),
            64 => Ok(Self::N64),
            128 => Ok(Self::N128),
            256 => Ok(Self::N256),
            512 => Ok(Self::N512),
            1024 => Ok(Self::N1024),
            _ => Err(Status::invalid_argument("invalid avg samples")),
        }
    }
}

impl From<AvgSamples> for u32 {
    fn from(value: AvgSamples) -> Self {
        match value {
            AvgSamples::N1 => 1,
            AvgSamples::N4 => 4,
            AvgSamples::N16 => 16,
            AvgSamples::N64 => 64,
            AvgSamples::N128 => 128,
            AvgSamples::N256 => 256,
            AvgSamples::N512 => 512,
            AvgSamples::N1024 => 1024,
        }
    }
}

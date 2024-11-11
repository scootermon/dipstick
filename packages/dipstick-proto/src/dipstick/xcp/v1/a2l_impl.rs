use tonic::{Result, Status};

use super::{
    A2lByteOrder, A2lCharacteristic, A2lCompuMethod, A2lCompuTab, A2lCompuVtab, A2lConversionType,
    A2lFullCharacteristic, A2lMeasurement, A2lModule, A2lRecordLayout,
};
use crate::wkt::value::Kind as ValueKind;
use crate::wkt::Value;

pub trait A2lNamedObject {
    fn name(&self) -> &str;
}

macro_rules! impl_named_object {
    ($type:ty) => {
        impl A2lNamedObject for $type {
            fn name(&self) -> &str {
                &self.name
            }
        }
    };
    ($( $type:ty ),* $(,)?) => {
        $( impl_named_object!($type); )*
    };
}

impl_named_object!(
    A2lModule,
    A2lCharacteristic,
    A2lCompuMethod,
    A2lCompuTab,
    A2lCompuVtab,
    A2lMeasurement,
    A2lRecordLayout,
);

impl A2lModule {
    pub fn byte_order(&self) -> A2lByteOrder {
        match self.mod_common {
            Some(ref common) => common.byte_order(),
            None => A2lByteOrder::Unspecified,
        }
    }

    /// Returns the characteristic with the given name.
    pub fn find_characteristic(&self, name: &str) -> Option<&A2lCharacteristic> {
        find_by_name(&self.characteristic, name)
    }

    /// Returns the computation method with the given name.
    pub fn find_compu_method(&self, name: &str) -> Option<&A2lCompuMethod> {
        find_by_name(&self.compu_method, name)
    }

    /// Returns the computation table with the given name.
    pub fn find_compu_tab(&self, name: &str) -> Option<&A2lCompuTab> {
        find_by_name(&self.compu_tab, name)
    }

    /// Returns the computation value table with the given name.
    pub fn find_compu_vtab(&self, name: &str) -> Option<&A2lCompuVtab> {
        find_by_name(&self.compu_vtab, name)
    }

    /// Returns the measurement with the given name.
    pub fn find_measurement(&self, name: &str) -> Option<&A2lMeasurement> {
        find_by_name(&self.measurement, name)
    }

    /// Returns the record layout with the given name.
    pub fn find_record_layout(&self, name: &str) -> Option<&A2lRecordLayout> {
        find_by_name(&self.record_layout, name)
    }
}

fn find_by_name<'a, T: A2lNamedObject>(items: &'a [T], name: &str) -> Option<&'a T> {
    items.iter().find(|item| item.name() == name)
}

impl A2lCompuVtab {
    /// Returns the output value for the given input value.
    pub fn map_in_val(&self, in_val: f64) -> Option<&str> {
        self.value_pairs
            .iter()
            .find_map(|pair| (pair.in_val == in_val).then_some(&*pair.out_val))
    }

    /// Returns the input value for the given output value.
    pub fn map_out_val(&self, out: &str) -> Option<f64> {
        self.value_pairs
            .iter()
            .find_map(|pair| (pair.out_val == out).then_some(pair.in_val))
    }
}

impl A2lFullCharacteristic {
    pub fn convert_value_to_record(&self, value: Value) -> Result<Value> {
        let Some(compu_method) = &self.compu_method else {
            return Ok(value);
        };
        match compu_method.conversion_type() {
            A2lConversionType::Unspecified | A2lConversionType::Identical => Ok(value),
            A2lConversionType::TabVerb => match value.kind {
                Some(ValueKind::NumberValue(_)) => Ok(value),
                Some(ValueKind::StringValue(v)) => {
                    let Some(compu_vtab) = &self.compu_vtab else {
                        return Err(Status::not_found("referenced compu value table not found"));
                    };
                    let out_val = compu_vtab.map_out_val(&v).ok_or_else(|| {
                        Status::invalid_argument(format!(
                            "value {v:?} not found in value table in {}",
                            compu_vtab.name
                        ))
                    })?;
                    Ok(Value {
                        kind: Some(ValueKind::NumberValue(out_val)),
                    })
                }
                _ => Err(Status::invalid_argument(
                    "expected number or string value for verb table conversion",
                )),
            },
            _ => Err(Status::unimplemented("conversion type not supported")),
        }
    }

    pub fn convert_record_to_value(&self, value: Value) -> Result<Value> {
        let Some(compu_method) = &self.compu_method else {
            return Ok(value);
        };
        match compu_method.conversion_type() {
            A2lConversionType::Unspecified | A2lConversionType::Identical => Ok(value),
            A2lConversionType::TabVerb => match value.kind {
                Some(ValueKind::NumberValue(v)) => {
                    let Some(compu_vtab) = &self.compu_vtab else {
                        return Err(Status::not_found("referenced compu value table not found"));
                    };
                    let in_val = compu_vtab.map_in_val(v).ok_or_else(|| {
                        Status::invalid_argument(format!(
                            "value {v:?} not found in value table {:?}",
                            compu_vtab.name
                        ))
                    })?;
                    Ok(Value {
                        kind: Some(ValueKind::StringValue(in_val.to_owned())),
                    })
                }
                Some(ValueKind::StringValue(_)) => Ok(value),
                _ => Err(Status::invalid_argument(
                    "expected number or string value for verb table conversion",
                )),
            },
            _ => Err(Status::unimplemented("conversion type not supported")),
        }
    }
}

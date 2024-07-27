pub use prost_types::*;
pub use protoc_wkt::google::protobuf::FILE_DESCRIPTOR_SET;

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

macro_rules! impl_into_value {
    ($kind:ident from $ty:ty) => {
        impl IntoValue for $ty {
            fn into_value(self) -> Value {
                Value {
                    kind: Some(value::Kind::$kind(self.into())),
                }
            }
        }
    };
}

impl_into_value!(BoolValue from bool);
impl_into_value!(NumberValue from f32);
impl_into_value!(NumberValue from f64);
impl_into_value!(StringValue from String);
impl_into_value!(StringValue from &str);

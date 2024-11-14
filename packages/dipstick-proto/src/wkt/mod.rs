pub use prost_types::*;
pub use protoc_wkt::google::protobuf::FILE_DESCRIPTOR_SET;
pub use value::Kind as ValueKind;

pub use self::into_value::IntoValue;

mod into_value;

// @generated
/// Log event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEvent {
    /// Timestamp
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Log level of the event.
    #[prost(enumeration="LogLevel", tag="2")]
    pub level: i32,
    /// Target module that generated the event.
    #[prost(string, tag="3")]
    pub target: ::prost::alloc::string::String,
    /// Log message.
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
    /// Additional fields.
    #[prost(map="string, message", tag="5")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// Active spans.
    #[prost(message, repeated, tag="6")]
    pub spans: ::prost::alloc::vec::Vec<LogSpan>,
}
/// Log span
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSpan {
    /// Unique id of the span.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// Log level of the span.
    #[prost(enumeration="LogLevel", tag="5")]
    pub level: i32,
    /// User defined name of the span.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Target where the span was created.
    #[prost(string, tag="4")]
    pub target: ::prost::alloc::string::String,
    /// Fields.
    #[prost(map="string, message", tag="3")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
/// Log config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogConfig {
    /// Default log level.
    #[prost(enumeration="LogLevel", tag="1")]
    pub default_level: i32,
    /// Target-specific log levels.
    #[prost(map="string, enumeration(LogLevel)", tag="2")]
    pub target_filters: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Log level
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    /// Unspecified
    Unspecified = 0,
    /// Trace
    Trace = 1,
    /// Debug
    Debug = 2,
    /// Info
    Info = 3,
    /// Warn
    Warn = 4,
    /// Error
    Error = 5,
    /// Off. Only used for filters
    Off = 6,
}
impl LogLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LogLevel::Unspecified => "LOG_LEVEL_UNSPECIFIED",
            LogLevel::Trace => "LOG_LEVEL_TRACE",
            LogLevel::Debug => "LOG_LEVEL_DEBUG",
            LogLevel::Info => "LOG_LEVEL_INFO",
            LogLevel::Warn => "LOG_LEVEL_WARN",
            LogLevel::Error => "LOG_LEVEL_ERROR",
            LogLevel::Off => "LOG_LEVEL_OFF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "LOG_LEVEL_TRACE" => Some(Self::Trace),
            "LOG_LEVEL_DEBUG" => Some(Self::Debug),
            "LOG_LEVEL_INFO" => Some(Self::Info),
            "LOG_LEVEL_WARN" => Some(Self::Warn),
            "LOG_LEVEL_ERROR" => Some(Self::Error),
            "LOG_LEVEL_OFF" => Some(Self::Off),
            _ => None,
        }
    }
}
/// Shutdown request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownRequest {
}
/// Shutdown response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownResponse {
}
/// Version request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {
}
/// Version response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    /// The version of the server.
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
}
/// LogConfig request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogConfigRequest {
    /// The new logging configuration.
    /// If empty, the current configuration is returned.
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<LogConfig>,
}
/// LogConfig response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogConfigResponse {
    /// The current logging configuration.
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<LogConfig>,
}
/// LogSubscribe request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSubscribeRequest {
}
/// LogSubscribe response
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogSubscribeResponse {
    /// A log event.
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<LogEvent>,
}
/// Encoded file descriptor set for the `dipstick.core.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf9, 0x15, 0x0a, 0x1e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x63, 0x6f,
    0x72, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x10, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f,
    0x72, 0x65, 0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0xec, 0x02, 0x0a, 0x08, 0x4c, 0x6f, 0x67, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x12, 0x38, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x30, 0x0a, 0x05, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x64, 0x69, 0x70,
    0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x6f,
    0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x16, 0x0a,
    0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x74,
    0x61, 0x72, 0x67, 0x65, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x3e, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x26, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e,
    0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x46, 0x69, 0x65, 0x6c,
    0x64, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x12,
    0x2f, 0x0a, 0x05, 0x73, 0x70, 0x61, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19,
    0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76,
    0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x53, 0x70, 0x61, 0x6e, 0x52, 0x05, 0x73, 0x70, 0x61, 0x6e, 0x73,
    0x1a, 0x51, 0x0a, 0x0b, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12,
    0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65,
    0x79, 0x12, 0x2c, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x16, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a,
    0x02, 0x38, 0x01, 0x22, 0x89, 0x02, 0x0a, 0x07, 0x4c, 0x6f, 0x67, 0x53, 0x70, 0x61, 0x6e, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x30, 0x0a, 0x05, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a,
    0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76,
    0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x05, 0x6c, 0x65, 0x76, 0x65,
    0x6c, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x12, 0x3d, 0x0a,
    0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e,
    0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31,
    0x2e, 0x4c, 0x6f, 0x67, 0x53, 0x70, 0x61, 0x6e, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x1a, 0x51, 0x0a, 0x0b,
    0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b,
    0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x2c, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x22,
    0x81, 0x02, 0x0a, 0x09, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x3f, 0x0a,
    0x0d, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c,
    0x52, 0x0c, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x12, 0x55,
    0x0a, 0x0e, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63,
    0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x2e, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72,
    0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0d, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x46, 0x69,
    0x6c, 0x74, 0x65, 0x72, 0x73, 0x1a, 0x5c, 0x0a, 0x12, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x46,
    0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b,
    0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x30, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1a, 0x2e, 0x64,
    0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e,
    0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a,
    0x02, 0x38, 0x01, 0x2a, 0x9f, 0x01, 0x0a, 0x08, 0x4c, 0x6f, 0x67, 0x4c, 0x65, 0x76, 0x65, 0x6c,
    0x12, 0x19, 0x0a, 0x15, 0x4c, 0x4f, 0x47, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x5f, 0x55, 0x4e,
    0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x13, 0x0a, 0x0f, 0x4c,
    0x4f, 0x47, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x5f, 0x54, 0x52, 0x41, 0x43, 0x45, 0x10, 0x01,
    0x12, 0x13, 0x0a, 0x0f, 0x4c, 0x4f, 0x47, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x5f, 0x44, 0x45,
    0x42, 0x55, 0x47, 0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x4c, 0x4f, 0x47, 0x5f, 0x4c, 0x45, 0x56,
    0x45, 0x4c, 0x5f, 0x49, 0x4e, 0x46, 0x4f, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x4c, 0x4f, 0x47,
    0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x5f, 0x57, 0x41, 0x52, 0x4e, 0x10, 0x04, 0x12, 0x13, 0x0a,
    0x0f, 0x4c, 0x4f, 0x47, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x10, 0x05, 0x12, 0x11, 0x0a, 0x0d, 0x4c, 0x4f, 0x47, 0x5f, 0x4c, 0x45, 0x56, 0x45, 0x4c, 0x5f,
    0x4f, 0x46, 0x46, 0x10, 0x06, 0x4a, 0xdc, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x3c, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x01, 0x00, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x26, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x17, 0x0a, 0x02, 0x05, 0x00,
    0x12, 0x04, 0x07, 0x00, 0x16, 0x01, 0x1a, 0x0b, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x6c, 0x65, 0x76,
    0x65, 0x6c, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x07, 0x05, 0x0d, 0x0a,
    0x1a, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x1c, 0x1a, 0x0d, 0x20, 0x55,
    0x6e, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x14, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0b, 0x02, 0x16, 0x1a, 0x07, 0x20, 0x54, 0x72, 0x61, 0x63, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0b, 0x14, 0x15, 0x0a, 0x14, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0d, 0x02, 0x16, 0x1a, 0x07, 0x20, 0x44, 0x65, 0x62, 0x75, 0x67, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x14, 0x15, 0x0a, 0x13, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x02, 0x15, 0x1a, 0x06, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0f, 0x13, 0x14, 0x0a, 0x13, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x11, 0x02, 0x15, 0x1a, 0x06, 0x20, 0x57, 0x61, 0x72, 0x6e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x02, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x11, 0x13, 0x14, 0x0a, 0x14, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x13, 0x02, 0x16, 0x1a, 0x07, 0x20, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x13, 0x02,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x13, 0x14, 0x15, 0x0a,
    0x29, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x15, 0x02, 0x14, 0x1a, 0x1c, 0x20, 0x4f,
    0x66, 0x66, 0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x06, 0x01, 0x12, 0x03, 0x15, 0x02, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06,
    0x02, 0x12, 0x03, 0x15, 0x12, 0x13, 0x0a, 0x17, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x19, 0x00,
    0x26, 0x01, 0x1a, 0x0b, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x10, 0x0a, 0x18, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x2a, 0x1a, 0x0b, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x1b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x1c,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x28, 0x29, 0x0a,
    0x26, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x15, 0x1a, 0x19, 0x20, 0x4c,
    0x6f, 0x67, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x13,
    0x14, 0x0a, 0x36, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x02, 0x14, 0x1a, 0x29,
    0x20, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x1f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x1f, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x1f, 0x12, 0x13, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x21, 0x02, 0x15,
    0x1a, 0x0e, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x21, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x21, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x21, 0x13, 0x14, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x23, 0x02, 0x30, 0x1a, 0x14, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x23, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x23, 0x25, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x23, 0x2e, 0x2f, 0x0a, 0x1c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x25, 0x02, 0x1d, 0x1a, 0x0f, 0x20, 0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x73, 0x70,
    0x61, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03,
    0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x25, 0x0b,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x25, 0x13, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x25, 0x1b, 0x1c, 0x0a, 0x16, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x29, 0x00, 0x34, 0x01, 0x1a, 0x0a, 0x20, 0x4c, 0x6f, 0x67, 0x20,
    0x73, 0x70, 0x61, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x29, 0x08,
    0x0f, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x02, 0x10, 0x1a, 0x18,
    0x20, 0x55, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x70, 0x61, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x2b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x2b, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b,
    0x0e, 0x0f, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x02, 0x15, 0x1a,
    0x18, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x70, 0x61, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2d, 0x13, 0x14, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x02, 0x12,
    0x1a, 0x20, 0x20, 0x55, 0x73, 0x65, 0x72, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20,
    0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x61, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x09, 0x0d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2f, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x31, 0x02, 0x14, 0x1a, 0x24, 0x20, 0x54, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x61,
    0x6e, 0x20, 0x77, 0x61, 0x73, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x31, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x31, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x31, 0x12, 0x13, 0x0a, 0x16, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x04, 0x12, 0x03, 0x33, 0x02, 0x30, 0x1a, 0x09, 0x20, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x33, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x33, 0x25, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x33, 0x2e, 0x2f, 0x0a, 0x18, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x37, 0x00, 0x3c, 0x01, 0x1a, 0x0c, 0x20, 0x4c, 0x6f, 0x67, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x37, 0x08,
    0x11, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x39, 0x02, 0x1d, 0x1a, 0x14,
    0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x6c, 0x65, 0x76,
    0x65, 0x6c, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x39,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x0b, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x1b, 0x1c, 0x0a, 0x2a,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3b, 0x02, 0x2b, 0x1a, 0x1d, 0x20, 0x54, 0x61,
    0x72, 0x67, 0x65, 0x74, 0x2d, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6c, 0x6f,
    0x67, 0x20, 0x6c, 0x65, 0x76, 0x65, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x3b, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x3b, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x3b, 0x29, 0x2a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xe8, 0x0f, 0x0a,
    0x1e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76,
    0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x10, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76,
    0x31, 0x1a, 0x1e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x63, 0x6f, 0x72, 0x65,
    0x2f, 0x76, 0x31, 0x2f, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x11, 0x0a, 0x0f, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x22, 0x12, 0x0a, 0x10, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x10, 0x0a, 0x0e, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x2b, 0x0a, 0x0f, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x18, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x47, 0x0a, 0x10, 0x4c, 0x6f, 0x67, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x33, 0x0a, 0x06, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c,
    0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x22, 0x48, 0x0a, 0x11, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x33, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b,
    0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0x15, 0x0a, 0x13, 0x4c, 0x6f,
    0x67, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x22, 0x48, 0x0a, 0x14, 0x4c, 0x6f, 0x67, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x30, 0x0a, 0x05, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74,
    0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x52, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x32, 0xe7, 0x02, 0x0a, 0x0b,
    0x43, 0x6f, 0x72, 0x65, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x51, 0x0a, 0x08, 0x53,
    0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x12, 0x21, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68, 0x75, 0x74, 0x64,
    0x6f, 0x77, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x22, 0x2e, 0x64, 0x69, 0x70,
    0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68,
    0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x4e,
    0x0a, 0x07, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x20, 0x2e, 0x64, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x21, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x54,
    0x0a, 0x09, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x22, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c,
    0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x23, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e,
    0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x5f, 0x0a, 0x0c, 0x4c, 0x6f, 0x67, 0x53, 0x75, 0x62, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x12, 0x25, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c, 0x6f, 0x67, 0x53, 0x75, 0x62, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x26, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x4c,
    0x6f, 0x67, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x30, 0x01, 0x4a, 0xc7, 0x09, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x32, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x01, 0x00, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x28, 0x0a,
    0x26, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x06, 0x00, 0x11, 0x01, 0x1a, 0x1a, 0x20, 0x53, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03,
    0x06, 0x08, 0x13, 0x0a, 0x25, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x3b,
    0x1a, 0x18, 0x20, 0x53, 0x68, 0x75, 0x74, 0x73, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x08, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x08, 0x29, 0x39, 0x0a, 0x31, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x02,
    0x38, 0x1a, 0x24, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0b, 0x06, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x0b, 0x0e, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x27,
    0x36, 0x0a, 0x3a, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x02, 0x3e, 0x1a, 0x2d,
    0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x71,
    0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x27, 0x73, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x06, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0e, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x2b, 0x3c, 0x0a, 0x32, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x10, 0x02, 0x4e, 0x1a, 0x25, 0x20, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x27,
    0x73, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x10, 0x13, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x10, 0x31, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x10, 0x38, 0x4c, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x03, 0x14, 0x00, 0x1a, 0x1a, 0x12,
    0x20, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x17, 0x0a, 0x1e,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x16, 0x00, 0x1b, 0x1a, 0x13, 0x20, 0x53, 0x68, 0x75, 0x74,
    0x64, 0x6f, 0x77, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x16, 0x08, 0x18, 0x0a, 0x1c, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x19, 0x00, 0x19, 0x1a, 0x11, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x19, 0x08, 0x16, 0x0a, 0x1e, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1b, 0x00, 0x1e, 0x01,
    0x1a, 0x12, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x17,
    0x0a, 0x29, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x15, 0x1a, 0x1c, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x1d, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x1d, 0x13, 0x14, 0x0a, 0x1f, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x21, 0x00, 0x25,
    0x01, 0x1a, 0x13, 0x20, 0x4c, 0x6f, 0x67, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x21,
    0x08, 0x18, 0x0a, 0x5f, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x17, 0x1a,
    0x52, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e,
    0x67, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x0a, 0x20, 0x49, 0x66, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x02,
    0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x0c, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x15, 0x16, 0x0a, 0x20, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01, 0x1a, 0x14, 0x20, 0x4c, 0x6f, 0x67, 0x43,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x27, 0x08, 0x19, 0x0a, 0x31, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x17, 0x1a, 0x24, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63,
    0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x0c, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x15, 0x16, 0x0a, 0x21, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x2d, 0x00, 0x1e, 0x1a, 0x16, 0x20, 0x4c, 0x6f, 0x67, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x06, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x1b, 0x0a, 0x23, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x2f,
    0x00, 0x32, 0x01, 0x1a, 0x17, 0x20, 0x4c, 0x6f, 0x67, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x07, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x1c, 0x0a, 0x1b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00,
    0x12, 0x03, 0x31, 0x02, 0x15, 0x1a, 0x0e, 0x20, 0x41, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x13, 0x14, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("dipstick.core.v1.tonic.rs");
// @@protoc_insertion_point(module)
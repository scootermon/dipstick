// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignalSpec {
    #[prost(message, optional, tag="1")]
    pub meta: ::core::option::Option<super::super::core::v1::EntityMeta>,
    #[prost(oneof="signal_spec::SignalSpecVariant", tags="3, 2, 4")]
    pub signal_spec_variant: ::core::option::Option<signal_spec::SignalSpecVariant>,
}
/// Nested message and enum types in `SignalSpec`.
pub mod signal_spec {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SignalSpecVariant {
        #[prost(message, tag="3")]
        A2lMeasurement(super::A2lMeasurementSignalSpec),
        #[prost(message, tag="2")]
        DeviceSensor(super::DeviceSensorSignalSpec),
        #[prost(message, tag="4")]
        Gpio(super::GpioSignalSpec),
    }
}
impl ::prost::Name for SignalSpec {
const NAME: &'static str = "SignalSpec";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct A2lMeasurementSignalSpec {
    #[prost(message, optional, tag="1")]
    pub a2l: ::core::option::Option<super::super::core::v1::EntitySelector>,
    #[prost(string, tag="2")]
    pub measurement: ::prost::alloc::string::String,
}
impl ::prost::Name for A2lMeasurementSignalSpec {
const NAME: &'static str = "A2lMeasurementSignalSpec";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSensorSignalSpec {
    #[prost(message, optional, tag="1")]
    pub device: ::core::option::Option<super::super::core::v1::EntitySelector>,
    #[prost(string, tag="2")]
    pub sensor: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub poll_interval: ::core::option::Option<::prost_types::Duration>,
}
impl ::prost::Name for DeviceSensorSignalSpec {
const NAME: &'static str = "DeviceSensorSignalSpec";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpioSignalSpec {
    #[prost(message, optional, tag="1")]
    pub chip: ::core::option::Option<super::super::core::v1::EntitySelector>,
    #[prost(string, tag="2")]
    pub pin: ::prost::alloc::string::String,
}
impl ::prost::Name for GpioSignalSpec {
const NAME: &'static str = "GpioSignalSpec";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShadowSpec {
    #[prost(message, optional, tag="1")]
    pub meta: ::core::option::Option<super::super::core::v1::EntityMeta>,
    #[prost(message, repeated, tag="2")]
    pub signal: ::prost::alloc::vec::Vec<SignalSpec>,
}
impl ::prost::Name for ShadowSpec {
const NAME: &'static str = "ShadowSpec";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShadowRequest {
    #[prost(message, optional, tag="1")]
    pub spec: ::core::option::Option<ShadowSpec>,
}
impl ::prost::Name for CreateShadowRequest {
const NAME: &'static str = "CreateShadowRequest";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShadowResponse {
    #[prost(message, optional, tag="1")]
    pub meta: ::core::option::Option<super::super::core::v1::EntityMeta>,
}
impl ::prost::Name for CreateShadowResponse {
const NAME: &'static str = "CreateShadowResponse";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShadowSignalEventsRequest {
    #[prost(message, optional, tag="1")]
    pub entity: ::core::option::Option<super::super::core::v1::EntitySelector>,
}
impl ::prost::Name for ShadowSignalEventsRequest {
const NAME: &'static str = "ShadowSignalEventsRequest";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShadowSignalEventsResponse {
}
impl ::prost::Name for ShadowSignalEventsResponse {
const NAME: &'static str = "ShadowSignalEventsResponse";
const PACKAGE: &'static str = "dipstick.shadow.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.shadow.v1.{}", Self::NAME)
            }}
/// Encoded file descriptor set for the `dipstick.shadow.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd3, 0x0d, 0x0a, 0x1f, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x73, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73,
    0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x65, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbb, 0x02, 0x0a, 0x0a, 0x53, 0x69, 0x67, 0x6e,
    0x61, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x12, 0x30, 0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65,
    0x74, 0x61, 0x52, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x12, 0x57, 0x0a, 0x0f, 0x61, 0x32, 0x6c, 0x5f,
    0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x2c, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61,
    0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x41, 0x32, 0x6c, 0x4d, 0x65, 0x61, 0x73, 0x75, 0x72,
    0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x48,
    0x00, 0x52, 0x0e, 0x61, 0x32, 0x6c, 0x4d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x12, 0x51, 0x0a, 0x0d, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x65, 0x6e, 0x73,
    0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2a, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74,
    0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c,
    0x53, 0x70, 0x65, 0x63, 0x48, 0x00, 0x52, 0x0c, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x65,
    0x6e, 0x73, 0x6f, 0x72, 0x12, 0x38, 0x0a, 0x04, 0x67, 0x70, 0x69, 0x6f, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x22, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x70, 0x69, 0x6f, 0x53, 0x69, 0x67, 0x6e,
    0x61, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x48, 0x00, 0x52, 0x04, 0x67, 0x70, 0x69, 0x6f, 0x42, 0x15,
    0x0a, 0x13, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x5f, 0x73, 0x70, 0x65, 0x63, 0x5f, 0x76, 0x61,
    0x72, 0x69, 0x61, 0x6e, 0x74, 0x22, 0x70, 0x0a, 0x18, 0x41, 0x32, 0x6c, 0x4d, 0x65, 0x61, 0x73,
    0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x53, 0x70, 0x65,
    0x63, 0x12, 0x32, 0x0a, 0x03, 0x61, 0x32, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20,
    0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76,
    0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x52, 0x03, 0x61, 0x32, 0x6c, 0x12, 0x20, 0x0a, 0x0b, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x6d, 0x65, 0x61, 0x73,
    0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x22, 0xaa, 0x01, 0x0a, 0x16, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x53, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x53, 0x70,
    0x65, 0x63, 0x12, 0x38, 0x0a, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f,
    0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x53, 0x65, 0x6c, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x12, 0x16, 0x0a, 0x06,
    0x73, 0x65, 0x6e, 0x73, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x65,
    0x6e, 0x73, 0x6f, 0x72, 0x12, 0x3e, 0x0a, 0x0d, 0x70, 0x6f, 0x6c, 0x6c, 0x5f, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x44, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0c, 0x70, 0x6f, 0x6c, 0x6c, 0x49, 0x6e, 0x74, 0x65,
    0x72, 0x76, 0x61, 0x6c, 0x22, 0x58, 0x0a, 0x0e, 0x47, 0x70, 0x69, 0x6f, 0x53, 0x69, 0x67, 0x6e,
    0x61, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x12, 0x34, 0x0a, 0x04, 0x63, 0x68, 0x69, 0x70, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x04, 0x63, 0x68, 0x69, 0x70, 0x12, 0x10, 0x0a, 0x03,
    0x70, 0x69, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x70, 0x69, 0x6e, 0x42, 0x8f,
    0x01, 0x0a, 0x16, 0x63, 0x6f, 0x6d, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x42, 0x0b, 0x53, 0x69, 0x67, 0x6e, 0x61,
    0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x44, 0x53, 0x58, 0xaa, 0x02,
    0x12, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x2e, 0x56, 0x31, 0xca, 0x02, 0x12, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53,
    0x68, 0x61, 0x64, 0x6f, 0x77, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1e, 0x44, 0x69, 0x70, 0x73, 0x74,
    0x69, 0x63, 0x6b, 0x5c, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50,
    0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x14, 0x44, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x3a, 0x3a, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x3a, 0x3a, 0x56, 0x31,
    0x4a, 0x8b, 0x06, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c,
    0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x1b, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x04, 0x00, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x07, 0x1e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x07, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x08, 0x02,
    0x0c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x31, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x09, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x1d, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x09, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0a, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0a,
    0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x1b, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x2b, 0x2c, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0b, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x0b, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0b, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0f, 0x00, 0x12,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x10, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x10, 0x22, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x10, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x11, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x09, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x02, 0x12, 0x04, 0x14, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x14, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x15, 0x02, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x22, 0x28, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x16, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x16, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x12,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x17, 0x02, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x17, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x1b, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04,
    0x1a, 0x00, 0x1d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x02, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x22, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1b, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x1c, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1c,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x09, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x0f, 0x10, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xd0, 0x04, 0x0a, 0x1f, 0x64, 0x69, 0x70, 0x73, 0x74,
    0x69, 0x63, 0x6b, 0x2f, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x64, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x1a, 0x1d,
    0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31,
    0x2f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x64,
    0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2f, 0x76,
    0x31, 0x2f, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x76,
    0x0a, 0x0a, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x53, 0x70, 0x65, 0x63, 0x12, 0x30, 0x0a, 0x04,
    0x6d, 0x65, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x69, 0x70,
    0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x74, 0x61, 0x52, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x12, 0x36,
    0x0a, 0x06, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x2e, 0x76, 0x31, 0x2e, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x52, 0x06,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x42, 0x8f, 0x01, 0x0a, 0x16, 0x63, 0x6f, 0x6d, 0x2e, 0x64,
    0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76,
    0x31, 0x42, 0x0b, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0xa2, 0x02, 0x03, 0x44, 0x53, 0x58, 0xaa, 0x02, 0x12, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63,
    0x6b, 0x2e, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x12, 0x44, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x5c, 0x56, 0x31,
    0xe2, 0x02, 0x1e, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53, 0x68, 0x61, 0x64,
    0x6f, 0x77, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0xea, 0x02, 0x14, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x3a, 0x3a, 0x53, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xc6, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x09, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x00, 0x29, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x06, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07,
    0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x02, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x1e, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x08, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x08, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x1f,
    0x20, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xd4, 0x09, 0x0a, 0x20, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2f, 0x76, 0x31,
    0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12,
    0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e,
    0x76, 0x31, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x63, 0x6f, 0x72,
    0x65, 0x2f, 0x76, 0x31, 0x2f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x1f, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x73, 0x68, 0x61, 0x64,
    0x6f, 0x77, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x49, 0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x68, 0x61, 0x64,
    0x6f, 0x77, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x32, 0x0a, 0x04, 0x73, 0x70, 0x65,
    0x63, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68, 0x61,
    0x64, 0x6f, 0x77, 0x53, 0x70, 0x65, 0x63, 0x52, 0x04, 0x73, 0x70, 0x65, 0x63, 0x22, 0x48, 0x0a,
    0x14, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x30, 0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63,
    0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x74,
    0x61, 0x52, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x22, 0x55, 0x0a, 0x19, 0x53, 0x68, 0x61, 0x64, 0x6f,
    0x77, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x38, 0x0a, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x53, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x06, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0x1c,
    0x0a, 0x1a, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0xe9, 0x01, 0x0a,
    0x0d, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x61,
    0x0a, 0x0c, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x12, 0x27,
    0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x75, 0x0a, 0x12, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x53, 0x69, 0x67, 0x6e, 0x61,
    0x6c, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x2d, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68, 0x61,
    0x64, 0x6f, 0x77, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2e, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63,
    0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68, 0x61, 0x64,
    0x6f, 0x77, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x30, 0x01, 0x42, 0x90, 0x01, 0x0a, 0x16, 0x63, 0x6f, 0x6d,
    0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x2e, 0x76, 0x31, 0x42, 0x0c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x44, 0x53, 0x58, 0xaa, 0x02, 0x12, 0x44, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2e, 0x56, 0x31, 0xca, 0x02,
    0x12, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1e, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53,
    0x68, 0x61, 0x64, 0x6f, 0x77, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x14, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x3a,
    0x3a, 0x53, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xca, 0x03, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x18, 0x25, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x1b, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x03, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x00, 0x29,
    0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0a, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x07, 0x02, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x07, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x07, 0x13,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x31, 0x45, 0x0a,
    0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x60, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x06, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x09, 0x19, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x09, 0x3d, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x09, 0x44, 0x5e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x0e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0d, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0d, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0d, 0x14, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x12, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x11, 0x1e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11,
    0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x14, 0x00, 0x16, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x15, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x15, 0x22, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x2b,
    0x2c, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x18, 0x00, 0x25, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x03, 0x01, 0x12, 0x03, 0x18, 0x08, 0x22, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("dipstick.shadow.v1.tonic.rs");
// @@protoc_insertion_point(module)
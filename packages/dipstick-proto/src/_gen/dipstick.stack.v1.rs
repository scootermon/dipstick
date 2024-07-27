// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackEntity {
    #[prost(message, optional, tag="1")]
    pub meta: ::core::option::Option<super::super::core::v1::EntityMeta>,
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<StackSpec>,
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<StackStatus>,
}
impl ::prost::Name for StackEntity {
const NAME: &'static str = "StackEntity";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackStatus {
}
impl ::prost::Name for StackStatus {
const NAME: &'static str = "StackStatus";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StackSpec {
    #[prost(message, optional, tag="8")]
    pub can: ::core::option::Option<CanPackageSpec>,
    #[prost(message, optional, tag="6")]
    pub device: ::core::option::Option<DevicePackageSpec>,
    #[prost(message, optional, tag="4")]
    pub gpio: ::core::option::Option<GpioPackageSpec>,
    #[prost(message, optional, tag="7")]
    pub shadow: ::core::option::Option<ShadowPackageSpec>,
    #[prost(message, optional, tag="3")]
    pub spi: ::core::option::Option<SpiPackageSpec>,
    #[prost(message, optional, tag="5")]
    pub xcp: ::core::option::Option<XcpPackageSpec>,
}
impl ::prost::Name for StackSpec {
const NAME: &'static str = "StackSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CanPackageSpec {
    #[prost(message, repeated, tag="1")]
    pub bus: ::prost::alloc::vec::Vec<super::super::can::v1::CreateBusRequest>,
}
impl ::prost::Name for CanPackageSpec {
const NAME: &'static str = "CanPackageSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevicePackageSpec {
    #[prost(message, repeated, tag="1")]
    pub device: ::prost::alloc::vec::Vec<super::super::device::v1::CreateDeviceRequest>,
}
impl ::prost::Name for DevicePackageSpec {
const NAME: &'static str = "DevicePackageSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpioPackageSpec {
    #[prost(message, repeated, tag="1")]
    pub chip: ::prost::alloc::vec::Vec<super::super::gpio::v1::CreateChipRequest>,
}
impl ::prost::Name for GpioPackageSpec {
const NAME: &'static str = "GpioPackageSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShadowPackageSpec {
    #[prost(message, repeated, tag="2")]
    pub shadow: ::prost::alloc::vec::Vec<super::super::shadow::v1::CreateShadowRequest>,
}
impl ::prost::Name for ShadowPackageSpec {
const NAME: &'static str = "ShadowPackageSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpiPackageSpec {
    #[prost(message, repeated, tag="1")]
    pub device: ::prost::alloc::vec::Vec<super::super::spi::v1::CreateDeviceRequest>,
}
impl ::prost::Name for SpiPackageSpec {
const NAME: &'static str = "SpiPackageSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XcpPackageSpec {
    #[prost(message, repeated, tag="1")]
    pub a2l: ::prost::alloc::vec::Vec<super::super::xcp::v1::CreateA2lRequest>,
    #[prost(message, repeated, tag="2")]
    pub session: ::prost::alloc::vec::Vec<super::super::xcp::v1::CreateSessionRequest>,
}
impl ::prost::Name for XcpPackageSpec {
const NAME: &'static str = "XcpPackageSpec";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStackRequest {
    #[prost(message, optional, tag="1")]
    pub meta: ::core::option::Option<super::super::core::v1::EntityMetaSpec>,
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<StackSpec>,
}
impl ::prost::Name for CreateStackRequest {
const NAME: &'static str = "CreateStackRequest";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStackResponse {
    #[prost(message, optional, tag="1")]
    pub stack: ::core::option::Option<StackEntity>,
}
impl ::prost::Name for CreateStackResponse {
const NAME: &'static str = "CreateStackResponse";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStackRequest {
    #[prost(message, optional, tag="1")]
    pub selector: ::core::option::Option<super::super::core::v1::EntitySelector>,
}
impl ::prost::Name for GetStackRequest {
const NAME: &'static str = "GetStackRequest";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStackResponse {
    #[prost(message, optional, tag="1")]
    pub stack: ::core::option::Option<StackEntity>,
}
impl ::prost::Name for GetStackResponse {
const NAME: &'static str = "GetStackResponse";
const PACKAGE: &'static str = "dipstick.stack.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("dipstick.stack.v1.{}", Self::NAME)
            }}
/// Encoded file descriptor set for the `dipstick.stack.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xef, 0x15, 0x0a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x73, 0x74,
    0x61, 0x63, 0x6b, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x11, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61,
    0x63, 0x6b, 0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f,
    0x63, 0x61, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x63,
    0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f, 0x64, 0x65,
    0x76, 0x69, 0x63, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f,
    0x67, 0x70, 0x69, 0x6f, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f,
    0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63,
    0x6b, 0x2f, 0x73, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b,
    0x2f, 0x78, 0x63, 0x70, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa9, 0x01, 0x0a, 0x0b, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x45,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x30, 0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63,
    0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x74,
    0x61, 0x52, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x12, 0x30, 0x0a, 0x04, 0x73, 0x70, 0x65, 0x63, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b,
    0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x53,
    0x70, 0x65, 0x63, 0x52, 0x04, 0x73, 0x70, 0x65, 0x63, 0x12, 0x36, 0x0a, 0x06, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74,
    0x61, 0x63, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x22, 0x0d, 0x0a, 0x0b, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x22, 0xde, 0x02, 0x0a, 0x09, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x53, 0x70, 0x65, 0x63, 0x12, 0x33,
    0x0a, 0x03, 0x63, 0x61, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e,
    0x43, 0x61, 0x6e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x52, 0x03,
    0x63, 0x61, 0x6e, 0x12, 0x3c, 0x0a, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x18, 0x06, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73,
    0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x50, 0x61,
    0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x52, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x12, 0x36, 0x0a, 0x04, 0x67, 0x70, 0x69, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x22, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b,
    0x2e, 0x76, 0x31, 0x2e, 0x47, 0x70, 0x69, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53,
    0x70, 0x65, 0x63, 0x52, 0x04, 0x67, 0x70, 0x69, 0x6f, 0x12, 0x3c, 0x0a, 0x06, 0x73, 0x68, 0x61,
    0x64, 0x6f, 0x77, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x64, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x52,
    0x06, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x12, 0x33, 0x0a, 0x03, 0x73, 0x70, 0x69, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x70, 0x69, 0x50, 0x61, 0x63, 0x6b,
    0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x52, 0x03, 0x73, 0x70, 0x69, 0x12, 0x33, 0x0a, 0x03,
    0x78, 0x63, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x64, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x58, 0x63,
    0x70, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x52, 0x03, 0x78, 0x63,
    0x70, 0x22, 0x45, 0x0a, 0x0e, 0x43, 0x61, 0x6e, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53,
    0x70, 0x65, 0x63, 0x12, 0x33, 0x0a, 0x03, 0x62, 0x75, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x21, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x61, 0x6e, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x42, 0x75, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x03, 0x62, 0x75, 0x73, 0x22, 0x54, 0x0a, 0x11, 0x44, 0x65, 0x76, 0x69,
    0x63, 0x65, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x12, 0x3f, 0x0a,
    0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x27, 0x2e,
    0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x22, 0x4a,
    0x0a, 0x0f, 0x47, 0x70, 0x69, 0x6f, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65,
    0x63, 0x12, 0x37, 0x0a, 0x04, 0x63, 0x68, 0x69, 0x70, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x23, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x67, 0x70, 0x69, 0x6f, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x43, 0x68, 0x69, 0x70, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x52, 0x04, 0x63, 0x68, 0x69, 0x70, 0x22, 0x54, 0x0a, 0x11, 0x53, 0x68,
    0x61, 0x64, 0x6f, 0x77, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70, 0x65, 0x63, 0x12,
    0x3f, 0x0a, 0x06, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x27, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x68, 0x61, 0x64, 0x6f,
    0x77, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x68, 0x61, 0x64, 0x6f,
    0x77, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x06, 0x73, 0x68, 0x61, 0x64, 0x6f, 0x77,
    0x22, 0x4e, 0x0a, 0x0e, 0x53, 0x70, 0x69, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53, 0x70,
    0x65, 0x63, 0x12, 0x3c, 0x0a, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x24, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x70,
    0x69, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x44, 0x65, 0x76, 0x69, 0x63,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x06, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65,
    0x22, 0x86, 0x01, 0x0a, 0x0e, 0x58, 0x63, 0x70, 0x50, 0x61, 0x63, 0x6b, 0x61, 0x67, 0x65, 0x53,
    0x70, 0x65, 0x63, 0x12, 0x33, 0x0a, 0x03, 0x61, 0x32, 0x6c, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x21, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x78, 0x63, 0x70, 0x2e,
    0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x41, 0x32, 0x6c, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x03, 0x61, 0x32, 0x6c, 0x12, 0x3f, 0x0a, 0x07, 0x73, 0x65, 0x73, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x64, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x2e, 0x78, 0x63, 0x70, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61,
    0x74, 0x65, 0x53, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x52, 0x07, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x42, 0x89, 0x01, 0x0a, 0x15, 0x63, 0x6f,
    0x6d, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b,
    0x2e, 0x76, 0x31, 0x42, 0x0a, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x01, 0xa2, 0x02, 0x03, 0x44, 0x53, 0x58, 0xaa, 0x02, 0x11, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x11, 0x44, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x5c, 0x56, 0x31, 0xe2,
    0x02, 0x1d, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53, 0x74, 0x61, 0x63, 0x6b,
    0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x13, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x3a, 0x3a, 0x53, 0x74, 0x61, 0x63,
    0x6b, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0x92, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x33, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x01, 0x00, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x27, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x05, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x06, 0x00, 0x28,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x07, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x08, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x03, 0x09, 0x00,
    0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0c, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x0c, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x1e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x25, 0x26,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0d, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x0c, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0d, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0e, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0e,
    0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x0e, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x17, 0x18, 0x0a, 0x09,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x00, 0x16, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x11, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x13, 0x00, 0x1a,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x14, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x14, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x14, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x15, 0x02, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x14, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x16, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x16, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x16, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16,
    0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x02, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x17, 0x02, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x17, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x18, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x18, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18,
    0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x18, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19, 0x02, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x19, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x19, 0x11, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x19, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1c,
    0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x0b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1d, 0x2c, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1d, 0x32, 0x33, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x20, 0x00, 0x22, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x20, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x21, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x21, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x21, 0x0b, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x21, 0x32, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x3b,
    0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x24, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x24, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02,
    0x00, 0x12, 0x03, 0x25, 0x02, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x25,
    0x0b, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x2e, 0x32,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x35, 0x36, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x28, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x29, 0x02, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x0b, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x32, 0x38, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x3b, 0x3c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x07, 0x12, 0x04, 0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03,
    0x2c, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x3a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x0b, 0x2e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x2f, 0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x38, 0x39, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04,
    0x30, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x30, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x34, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x0b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x31, 0x2c, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x31, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x32,
    0x02, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x03, 0x32, 0x0b, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x30, 0x37, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x3a, 0x3b, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x33, 0x0a, 0xe0, 0x0a, 0x0a, 0x1f, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2f,
    0x73, 0x74, 0x61, 0x63, 0x6b, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b,
    0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74,
    0x69, 0x63, 0x6b, 0x2f, 0x63, 0x6f, 0x72, 0x65, 0x2f, 0x76, 0x31, 0x2f, 0x65, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2f, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x74, 0x61, 0x63,
    0x6b, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7c, 0x0a, 0x12, 0x43, 0x72, 0x65, 0x61, 0x74,
    0x65, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x34, 0x0a,
    0x04, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x4d, 0x65, 0x74, 0x61, 0x53, 0x70, 0x65, 0x63, 0x52, 0x04, 0x6d,
    0x65, 0x74, 0x61, 0x12, 0x30, 0x0a, 0x04, 0x73, 0x70, 0x65, 0x63, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61,
    0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x53, 0x70, 0x65, 0x63, 0x52,
    0x04, 0x73, 0x70, 0x65, 0x63, 0x22, 0x4b, 0x0a, 0x13, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53,
    0x74, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x05,
    0x73, 0x74, 0x61, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64, 0x69,
    0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e,
    0x53, 0x74, 0x61, 0x63, 0x6b, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x52, 0x05, 0x73, 0x74, 0x61,
    0x63, 0x6b, 0x22, 0x4f, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3c, 0x0a, 0x08, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x52, 0x08, 0x73, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x22, 0x48, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x63, 0x6b,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63,
    0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x61, 0x63, 0x6b,
    0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x52, 0x05, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x32, 0xc1, 0x01,
    0x0a, 0x0c, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x5c,
    0x0a, 0x0b, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x12, 0x25, 0x2e,
    0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76,
    0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x1a, 0x26, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e,
    0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x53,
    0x74, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x53, 0x0a, 0x08,
    0x47, 0x65, 0x74, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x12, 0x22, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74,
    0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74,
    0x53, 0x74, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e, 0x64,
    0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31,
    0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x42, 0x8b, 0x01, 0x0a, 0x15, 0x63, 0x6f, 0x6d, 0x2e, 0x64, 0x69, 0x70, 0x73, 0x74, 0x69,
    0x63, 0x6b, 0x2e, 0x73, 0x74, 0x61, 0x63, 0x6b, 0x2e, 0x76, 0x31, 0x42, 0x0c, 0x53, 0x65, 0x72,
    0x76, 0x69, 0x63, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x44, 0x53,
    0x58, 0xaa, 0x02, 0x11, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b, 0x2e, 0x53, 0x74, 0x61,
    0x63, 0x6b, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x11, 0x44, 0x69, 0x70, 0x73, 0x74, 0x69, 0x63, 0x6b,
    0x5c, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x1d, 0x44, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x5c, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50,
    0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x13, 0x44, 0x69, 0x70, 0x73,
    0x74, 0x69, 0x63, 0x6b, 0x3a, 0x3a, 0x53, 0x74, 0x61, 0x63, 0x6b, 0x3a, 0x3a, 0x56, 0x31, 0x4a,
    0xab, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x1a, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x04, 0x00, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x06, 0x00, 0x09, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x44, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x07, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x07, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x07, 0x2f, 0x42, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x02, 0x3b,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08, 0x06, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x08, 0x0f, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x29, 0x39, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0c, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x22, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x0d, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x0d, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d,
    0x0c, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x13, 0x14,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x10, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x10, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x11, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x11, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x0e,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x16, 0x17, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x14, 0x00, 0x16, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x15, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x15,
    0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x22, 0x2a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x2d, 0x2e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x18, 0x00, 0x1a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x18, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x19, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x02,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x19, 0x0e, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x19, 0x16, 0x17, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("dipstick.stack.v1.tonic.rs");
// @@protoc_insertion_point(module)
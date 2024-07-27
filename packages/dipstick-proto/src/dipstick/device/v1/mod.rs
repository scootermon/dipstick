include!("../../../_gen/dipstick.device.v1.rs");

pub use self::device_event::DeviceEventVariant;
pub use self::device_service_client::DeviceServiceClient;
pub use self::device_service_server::{DeviceService, DeviceServiceServer};
pub use self::device_spec::DeviceSpecVariant;
pub use self::ina2xx_spec::Ina2xxTransportSpec;

mod event_impl;

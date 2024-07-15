include!("../../../_gen/dipstick.gpio.v1.rs");

pub use chip_spec::ChipSpecVariant;
pub use gpio_service_client::GpioServiceClient;
pub use gpio_service_server::{GpioService, GpioServiceServer};

impl From<Level> for crate::wkt::Value {
    fn from(level: Level) -> Self {
        let kind = match level {
            Level::Unspecified => {
                crate::wkt::value::Kind::NullValue(crate::wkt::NullValue::NullValue as _)
            }
            Level::Low => crate::wkt::value::Kind::BoolValue(false),
            Level::High => crate::wkt::value::Kind::BoolValue(true),
        };
        Self { kind: Some(kind) }
    }
}

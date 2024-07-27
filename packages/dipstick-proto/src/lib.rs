pub use dipstick::*;
pub use prost::{Message, Name};

mod dipstick;
pub mod wkt;

pub const ALL_FILE_DESCRIPTOR_SETS: &[&[u8]] = &[
    crate::wkt::FILE_DESCRIPTOR_SET,
    crate::can::v1::FILE_DESCRIPTOR_SET,
    crate::core::v1::FILE_DESCRIPTOR_SET,
    crate::device::v1::FILE_DESCRIPTOR_SET,
    crate::gpio::v1::FILE_DESCRIPTOR_SET,
    crate::shadow::v1::FILE_DESCRIPTOR_SET,
    crate::spi::v1::FILE_DESCRIPTOR_SET,
    crate::stack::v1::FILE_DESCRIPTOR_SET,
    crate::xcp::v1::FILE_DESCRIPTOR_SET,
];

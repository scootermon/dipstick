#![allow(unused_mut)]

pub mod wkt {
    pub use prost_types::*;
    pub use protoc_wkt::google::protobuf::FILE_DESCRIPTOR_SET;
}

pub mod can {
    include!("_gen/dipstick.can.v1.rs");

    pub use self::can_service_client::CanServiceClient;
    pub use self::can_service_server::{CanService, CanServiceServer};
}

pub mod core {
    include!("_gen/dipstick.core.v1.rs");

    pub use self::core_service_client::CoreServiceClient;
    pub use self::core_service_server::{CoreService, CoreServiceServer};
}

pub mod isotp {
    include!("_gen/dipstick.isotp.v1.rs");

    pub use self::isotp_service_client::IsotpServiceClient;
    pub use self::isotp_service_server::{IsotpService, IsotpServiceServer};
}

pub mod uds {
    include!("_gen/dipstick.uds.v1.rs");

    pub use self::uds_service_client::UdsServiceClient;
    pub use self::uds_service_server::{UdsService, UdsServiceServer};
}

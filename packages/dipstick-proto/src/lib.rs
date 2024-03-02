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
pub mod server {
    include!("_gen/dipstick.server.v1.rs");

    pub use self::server_service_client::ServerServiceClient;
    pub use self::server_service_server::{ServerService, ServerServiceServer};
}

pub mod uds {
    include!("_gen/dipstick.uds.v1.rs");

    pub use self::uds_service_client::UdsServiceClient;
    pub use self::uds_service_server::{UdsService, UdsServiceServer};
}

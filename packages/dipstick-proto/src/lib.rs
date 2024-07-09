#![allow(unused_mut)]

pub use dipstick::*;
pub use prost::Message;

mod dipstick {
    pub mod can {
        pub mod v1 {
            include!("_gen/dipstick.can.v1.rs");

            pub use self::can_service_client::CanServiceClient;
            pub use self::can_service_server::{CanService, CanServiceServer};
            pub use crate::can_impl::*;
        }

        pub mod isotp {
            pub mod v1 {
                include!("_gen/dipstick.can.isotp.v1.rs");

                pub use self::isotp_service_client::IsotpServiceClient;
                pub use self::isotp_service_server::{IsotpService, IsotpServiceServer};
            }
        }
    }

    pub mod core {
        pub mod v1 {
            include!("_gen/dipstick.core.v1.rs");

            pub use self::core_service_client::CoreServiceClient;
            pub use self::core_service_server::{CoreService, CoreServiceServer};
        }
    }

    pub mod shadow {
        pub mod v1 {
            include!("_gen/dipstick.shadow.v1.rs");

            pub use self::shadow_service_client::ShadowServiceClient;
            pub use self::shadow_service_server::{ShadowService, ShadowServiceServer};
        }
    }

    pub mod uds {
        pub mod v1 {
            include!("_gen/dipstick.uds.v1.rs");

            pub use self::uds_service_client::UdsServiceClient;
            pub use self::uds_service_server::{UdsService, UdsServiceServer};
        }
    }

    pub mod xcp {
        pub mod v1 {
            include!("_gen/dipstick.xcp.v1.rs");

            pub use self::cto_command::CtoCommandContent;
            pub use self::cto_response::CtoResponseContent;
            pub use self::frame::FrameContent;
            pub use self::xcp_service_client::XcpServiceClient;
            pub use self::xcp_service_server::{XcpService, XcpServiceServer};
            pub use crate::xcp_impl::*;
        }

        pub mod a2l {
            pub mod v1 {
                include!("_gen/dipstick.xcp.a2l.v1.rs");
            }
        }
    }
}

pub mod wkt {
    pub use prost_types::*;
    pub use protoc_wkt::google::protobuf::FILE_DESCRIPTOR_SET;
}

mod can_impl;
mod xcp_impl;

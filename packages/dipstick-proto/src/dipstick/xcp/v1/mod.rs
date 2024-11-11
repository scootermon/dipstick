include!("../../../_gen/dipstick.xcp.v1.rs");

pub use self::cto_req::CtoReqData;
pub use self::cto_resp::CtoRespData;
pub use self::session_spec::XcpTransportSpec;
pub use self::xcp_service_client::XcpServiceClient;
pub use self::xcp_service_server::{XcpService, XcpServiceServer};

mod a2l_impl;
mod cto_req_impl;

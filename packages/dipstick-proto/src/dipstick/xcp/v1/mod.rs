include!("../../../_gen/dipstick.xcp.v1.rs");

pub use self::cto_req::CtoReqData;
pub use self::cto_resp::CtoRespData;
pub use self::session_spec::XcpTransportSpec;
pub use self::xcp_service_client::XcpServiceClient;
pub use self::xcp_service_server::{XcpService, XcpServiceServer};

mod cto_req_impl;

impl A2lModule {
    pub fn byte_order(&self) -> A2lByteOrder {
        match self.mod_common {
            Some(ref common) => common.byte_order(),
            None => A2lByteOrder::Unspecified,
        }
    }
}

impl CtoReqData {
    pub const fn pid(&self) -> CtoReqPid {
        match self {
            Self::Connect(_) => CtoReqPid::Connect,
            Self::ShortUpload(_) => CtoReqPid::ShortUpload,
            Self::ShortDownload(_) => CtoReqPid::ShortDownload,
            Self::SetMta(_) => CtoReqPid::SetMta,
            Self::Download(_) => CtoReqPid::Download,
        }
    }
}

include!("../../../_gen/dipstick.xcp.v1.rs");

pub use self::cto_req::CtoReqData;
pub use self::cto_resp::CtoRespData;
pub use self::session_spec::XcpTransportSpec;
pub use self::xcp_service_client::XcpServiceClient;
pub use self::xcp_service_server::{XcpService, XcpServiceServer};

impl CtoReq {
    pub const fn pid(&self) -> CtoReqPid {
        match &self.cto_req_data {
            Some(data) => data.pid(),
            None => CtoReqPid::Unspecified,
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

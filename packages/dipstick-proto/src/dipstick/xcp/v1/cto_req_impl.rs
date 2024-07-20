use super::*;

impl CtoReq {
    pub const fn pid(&self) -> CtoReqPid {
        match &self.cto_req_data {
            Some(data) => data.pid(),
            None => CtoReqPid::Unspecified,
        }
    }
}

impl From<CtoConnectReqData> for CtoReq {
    fn from(value: CtoConnectReqData) -> Self {
        Self {
            cto_req_data: Some(CtoReqData::Connect(value)),
        }
    }
}

impl From<CtoShortUploadReqData> for CtoReq {
    fn from(value: CtoShortUploadReqData) -> Self {
        Self {
            cto_req_data: Some(CtoReqData::ShortUpload(value)),
        }
    }
}

impl From<CtoShortDownloadReqData> for CtoReq {
    fn from(value: CtoShortDownloadReqData) -> Self {
        Self {
            cto_req_data: Some(CtoReqData::ShortDownload(value)),
        }
    }
}

impl From<CtoSetMtaReqData> for CtoReq {
    fn from(value: CtoSetMtaReqData) -> Self {
        Self {
            cto_req_data: Some(CtoReqData::SetMta(value)),
        }
    }
}

impl From<CtoDownloadReqData> for CtoReq {
    fn from(value: CtoDownloadReqData) -> Self {
        Self {
            cto_req_data: Some(CtoReqData::Download(value)),
        }
    }
}

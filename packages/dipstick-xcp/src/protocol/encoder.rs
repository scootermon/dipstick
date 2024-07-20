use bytes::BufMut;
use dipstick_proto::xcp::v1::{
    ByteOrder, CtoConnectReqData, CtoDownloadReqData, CtoReq, CtoReqData, CtoReqPid,
    CtoSetMtaReqData, CtoShortDownloadReqData, CtoShortUploadReqData,
};
use tonic::{Result, Status};

pub fn cto_req(buf: &mut impl BufMut, byte_order: ByteOrder, req: &CtoReq) -> Result<()> {
    let req_data = req
        .cto_req_data
        .as_ref()
        .ok_or_else(|| Status::invalid_argument("missing cto req data"))?;
    cto_req_pid(buf, req_data.pid());
    match req_data {
        CtoReqData::Connect(req) => cto_connect_req_data(buf, req),
        CtoReqData::SetMta(req) => cto_set_mta_req_data(buf, byte_order, req),
        CtoReqData::ShortUpload(req) => cto_short_upload_req_data(buf, byte_order, req),
        CtoReqData::ShortDownload(req) => cto_short_download_req_data(buf, byte_order, req),
        CtoReqData::Download(req) => cto_download_req_data(buf, req),
    }
}

fn cto_connect_req_data(buf: &mut impl BufMut, req: &CtoConnectReqData) -> Result<()> {
    let mode = req
        .mode
        .try_into()
        .map_err(|_err| Status::invalid_argument("mode out of range"))?;
    buf.put_u8(mode);
    Ok(())
}

fn cto_set_mta_req_data(
    buf: &mut impl BufMut,
    byte_order: ByteOrder,
    req: &CtoSetMtaReqData,
) -> Result<()> {
    buf.put_u8(0);
    buf.put_u8(0);
    address_extension(buf, req.address_extension)?;
    u32(buf, byte_order, req.address);
    Ok(())
}

fn cto_short_upload_req_data(
    buf: &mut impl BufMut,
    byte_order: ByteOrder,
    req: &CtoShortUploadReqData,
) -> Result<()> {
    let length = req
        .length
        .try_into()
        .map_err(|_err| Status::invalid_argument("length out of range"))?;
    buf.put_u8(length);
    buf.put_u8(0);
    address_extension(buf, req.address_extension)?;
    u32(buf, byte_order, req.address);
    Ok(())
}

fn cto_short_download_req_data(
    buf: &mut impl BufMut,
    byte_order: ByteOrder,
    req: &CtoShortDownloadReqData,
) -> Result<()> {
    let length = req
        .data
        .len()
        .try_into()
        .map_err(|_err| Status::invalid_argument("data length out of range"))?;
    buf.put_u8(length);
    buf.put_u8(0);
    address_extension(buf, req.address_extension)?;
    u32(buf, byte_order, req.address);
    buf.put_slice(&req.data);
    Ok(())
}

fn cto_download_req_data(buf: &mut impl BufMut, req: &CtoDownloadReqData) -> Result<()> {
    let length: u8 = if req.block_mode_length > 0 {
        req.block_mode_length
            .try_into()
            .map_err(|_| Status::invalid_argument("block mode length out of range"))?
    } else {
        req.data
            .len()
            .try_into()
            .map_err(|_| Status::invalid_argument("data length out of range"))?
    };
    buf.put_u8(length);
    buf.put_slice(&req.data);
    Ok(())
}

#[inline]
fn address_extension(buf: &mut impl BufMut, address_extension: u32) -> Result<()> {
    buf.put_u8(
        address_extension
            .try_into()
            .map_err(|_err| Status::invalid_argument("address extension out of range"))?,
    );
    Ok(())
}

#[inline(always)]
fn u32(buf: &mut impl BufMut, byte_order: ByteOrder, value: u32) {
    match byte_order {
        ByteOrder::BigEndian => buf.put_u32(value),
        ByteOrder::LittleEndian => buf.put_u32_le(value),
        ByteOrder::Unspecified => unreachable!(),
    }
}

#[inline]
fn cto_req_pid(buf: &mut impl BufMut, pid: CtoReqPid) {
    const fn to_value(pid: CtoReqPid) -> u8 {
        match pid {
            CtoReqPid::Unspecified => unreachable!(),
            CtoReqPid::Connect => 0xFF,
            CtoReqPid::SetMta => 0xF6,
            CtoReqPid::ShortUpload => 0xF4,
            CtoReqPid::ShortDownload => 0xED,
            CtoReqPid::Download => 0xF0,
        }
    }
    buf.put_u8(to_value(pid));
}

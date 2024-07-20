use bytes::{Buf, Bytes};
use dipstick_proto::xcp::v1::{
    AddressGranularity, ByteOrder, CtoCommModeBasic, CtoConnectRespData, CtoErrorCode,
    CtoEventCode, CtoReqPid, CtoResourceType, CtoResp, CtoRespData, CtoRespPacket, CtoRespPid,
    CtoShortUploadRespData,
};

pub fn packet(input: &mut Bytes) -> anyhow::Result<CtoRespPacket> {
    anyhow::ensure!(input.remaining() >= 1, "insufficient data");
    let pid = cto_resp_pid(input.chunk()[0]);
    if pid != CtoRespPid::Unspecified {
        input.advance(1);
    }
    let event = if pid == CtoRespPid::Ev {
        anyhow::ensure!(input.remaining() >= 1, "missing event type");
        cto_event_code(input.get_u8())?
    } else {
        CtoEventCode::Unspecified
    };
    let error = if pid == CtoRespPid::Err {
        anyhow::ensure!(input.remaining() >= 1, "missing error code");
        cto_error_code(input.get_u8())?
    } else {
        CtoErrorCode::Unspecified
    };
    Ok(CtoRespPacket {
        timestamp: None,
        pid: pid as _,
        event: event as _,
        error: error as _,
        data: input.clone(),
    })
}

pub fn cto_resp(input: &mut Bytes, req_pid: CtoReqPid) -> anyhow::Result<CtoResp> {
    let data = match req_pid {
        CtoReqPid::Connect => cto_connect_resp_data(input).map(CtoRespData::Connect)?,
        CtoReqPid::ShortUpload => {
            cto_short_upload_resp_data(input).map(CtoRespData::ShortUpload)?
        }
        CtoReqPid::Unspecified => unreachable!(),
        _ => todo!(),
    };
    Ok(CtoResp {
        timestamp: None,
        cto_resp_data: Some(data),
    })
}

fn cto_connect_resp_data(input: &mut Bytes) -> anyhow::Result<CtoConnectRespData> {
    if input.remaining() < 7 {
        anyhow::bail!("insufficient data");
    }
    let resource_type = cto_resource_type(input.get_u8());
    let comm_mode_basic = cto_comm_mode_basic(input.get_u8());
    let max_cto = input.get_u8();
    let max_dto = u16(input, comm_mode_basic.byte_order());
    let protocol_layer_version = input.get_u8();
    let transport_layer_version = input.get_u8();

    Ok(CtoConnectRespData {
        resource: Some(resource_type),
        comm_mode_basic: Some(comm_mode_basic),
        max_cto: u32::from(max_cto),
        max_dto: u32::from(max_dto),
        protocol_layer_version: u32::from(protocol_layer_version),
        transport_layer_version: u32::from(transport_layer_version),
    })
}

fn cto_short_upload_resp_data(input: &mut Bytes) -> anyhow::Result<CtoShortUploadRespData> {
    Ok(CtoShortUploadRespData {
        data: input.clone(),
    })
}

#[inline(always)]
fn u16(buf: &mut impl Buf, byte_order: ByteOrder) -> u16 {
    match byte_order {
        ByteOrder::BigEndian => buf.get_u16(),
        ByteOrder::LittleEndian => buf.get_u16_le(),
        ByteOrder::Unspecified => unreachable!(),
    }
}

fn cto_resource_type(raw: u8) -> CtoResourceType {
    CtoResourceType {
        dbg: (raw >> 5) & 1 != 0,
        pgm: (raw >> 4) & 1 != 0,
        stim: (raw >> 3) & 1 != 0,
        daq: (raw >> 2) & 1 != 0,
        calpag: raw & 1 != 0,
    }
}

fn cto_comm_mode_basic(raw: u8) -> CtoCommModeBasic {
    let optional = (raw >> 7) & 1 != 0;
    let server_block_mode = (raw >> 6) & 1 != 0;
    let address_granularity = address_granularity((raw >> 1) & 0b11);
    let byte_order = byte_order(raw & 1 != 0);
    CtoCommModeBasic {
        optional,
        server_block_mode,
        address_granularity: address_granularity as _,
        byte_order: byte_order as _,
    }
}

fn address_granularity(bits: u8) -> AddressGranularity {
    match bits {
        0b00 => AddressGranularity::Byte,
        0b01 => AddressGranularity::Word,
        0b10 => AddressGranularity::Dword,
        0b11 => AddressGranularity::Reserved,
        _ => unreachable!(),
    }
}

fn byte_order(bit: bool) -> ByteOrder {
    if bit {
        ByteOrder::BigEndian
    } else {
        ByteOrder::LittleEndian
    }
}

fn cto_event_code(raw: u8) -> anyhow::Result<CtoEventCode> {
    match raw {
        0x00 => Ok(CtoEventCode::ResumeMode),
        0x01 => Ok(CtoEventCode::ClearDaq),
        0x02 => Ok(CtoEventCode::StoreDaq),
        0x03 => Ok(CtoEventCode::StoreCal),
        0x05 => Ok(CtoEventCode::CmdPending),
        0x06 => Ok(CtoEventCode::DaqOverload),
        0x07 => Ok(CtoEventCode::SessionTerminated),
        0x08 => Ok(CtoEventCode::TimeSync),
        0x09 => Ok(CtoEventCode::StimTimeout),
        0x0A => Ok(CtoEventCode::Sleep),
        0x0B => Ok(CtoEventCode::WakeUp),
        0xFE => Ok(CtoEventCode::User),
        0xFF => Ok(CtoEventCode::Transport),
        _ => anyhow::bail!("invalid event: {raw:#02x}"),
    }
}

fn cto_resp_pid(raw: u8) -> CtoRespPid {
    match raw {
        0xFC => CtoRespPid::Serv,
        0xFD => CtoRespPid::Ev,
        0xFE => CtoRespPid::Err,
        0xFF => CtoRespPid::Ok,
        _ => CtoRespPid::Unspecified,
    }
}

fn cto_error_code(raw: u8) -> anyhow::Result<CtoErrorCode> {
    match raw {
        0x00 => Ok(CtoErrorCode::CmdSynch),
        0x10 => Ok(CtoErrorCode::CmdBusy),
        0x11 => Ok(CtoErrorCode::DaqActive),
        0x12 => Ok(CtoErrorCode::PgmActive),
        0x20 => Ok(CtoErrorCode::CmdUnknown),
        0x21 => Ok(CtoErrorCode::CmdSyntax),
        0x22 => Ok(CtoErrorCode::OutOfRange),
        0x23 => Ok(CtoErrorCode::WriteProtected),
        0x24 => Ok(CtoErrorCode::AccessDenied),
        0x25 => Ok(CtoErrorCode::AccessLocked),
        0x26 => Ok(CtoErrorCode::PageNotValid),
        0x27 => Ok(CtoErrorCode::ModeNotValid),
        0x28 => Ok(CtoErrorCode::SegmentNotValid),
        0x29 => Ok(CtoErrorCode::Sequence),
        0x2A => Ok(CtoErrorCode::DaqConfig),
        0x30 => Ok(CtoErrorCode::MemoryOverflow),
        0x31 => Ok(CtoErrorCode::Generic),
        0x32 => Ok(CtoErrorCode::Verify),
        0x33 => Ok(CtoErrorCode::ResourceTemporaryNotAccessible),
        _ => anyhow::bail!("invalid error code: {raw:#02x}"),
    }
}

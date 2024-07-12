use bytes::{Buf, BufMut};
use dipstick_proto::xcp::v1::{
    AddressGranularity, ByteOrder, CommModeBasic, ConnectCommand, ConnectResponse,
    CtoCommandContent, DisconnectCommand, Resource, ShortUploadCommand,
};
use tonic::Status;

macro_rules! truncate {
    ($value:expr => $name:expr) => {
        ($value)
            .try_into()
            .map_err(|_| Status::invalid_argument(format!("'{}' out of range", $name)))?
    };
    ($container:ident . $name:ident) => {
        truncate!($container.$name => stringify!($name))
    };
    ($name:ident) => {
        truncate!($name => stringify!($name))
    };
}

macro_rules! bit {
    ($value:ident[$bit:literal]) => {
        ($value & (1 << $bit)) != 0
    };
    ($value:ident[$start:literal.. $end:literal]) => {
        ($value >> $start) & ((1 << ($end - $start)) - 1)
    };
}

pub fn encode_command(
    command: &CtoCommandContent,
    is_be: bool,
    buf: &mut impl BufMut,
) -> tonic::Result<()> {
    match *command {
        CtoCommandContent::Connect(ConnectCommand { mode }) => {
            buf.put_u8(cmd::CONNECT);
            buf.put_u8(truncate!(mode));
            Ok(())
        }
        CtoCommandContent::Disconnect(DisconnectCommand {}) => {
            buf.put_u8(cmd::DISCONNECT);
            Ok(())
        }
        CtoCommandContent::ShortUpload(ShortUploadCommand {
            length,
            address,
            address_extension,
        }) => {
            buf.put_u8(cmd::SHORT_UPLOAD);
            buf.put_u8(truncate!(length));
            buf.put_u8(0x00);
            buf.put_u8(truncate!(address_extension));
            encode_u32(address, is_be, buf);
            Ok(())
        }
    }
}

pub fn decode_response(buf: &mut impl Buf, is_be: bool) -> anyhow::Result<()> {
    let pid = buf.get_u8();
    match pid {
        // serv
        0xFC => {}
        // event
        0xFD => {}
        // response
        0xFE.. => {}
        // odt
        _ => {}
    }
    todo!()
}

pub fn decode_connect_response(buf: &mut impl Buf) -> anyhow::Result<ConnectResponse> {
    anyhow::ensure!(buf.remaining() >= 7, "response too short");
    let mut resp = ConnectResponse::default();
    resp.resource = Some({
        let raw = buf.get_u8();
        Resource {
            dbg: bit!(raw[5]),
            pgm: bit!(raw[4]),
            stim: bit!(raw[3]),
            daq: bit!(raw[2]),
            calpag: bit!(raw[0]),
        }
    });
    let byte_order;
    resp.comm_mode_basic = Some({
        let raw = buf.get_u8();
        byte_order = if bit!(raw[0]) {
            ByteOrder::Big
        } else {
            ByteOrder::Little
        };
        let address_granularity = match bit!(raw[2..0]) {
            0b00 => AddressGranularity::Byte,
            0b01 => AddressGranularity::Word,
            0b10 => AddressGranularity::Dword,
            _ => anyhow::bail!("address granularity invalid"),
        };
        CommModeBasic {
            optional: bit!(raw[7]),
            server_block_mode: bit!(raw[6]),
            address_granularity: address_granularity as _,
            byte_order: byte_order as _,
        }
    });
    resp.max_cto = u32::from(buf.get_u8());
    resp.max_dto = u32::from(decode_u16(buf, byte_order == ByteOrder::Big));
    resp.protocol_layer_version = u32::from(buf.get_u8());
    resp.transport_layer_version = u32::from(buf.get_u8());
    Ok(resp)
}

mod cmd {
    pub const CONNECT: u8 = 0xFF;
    pub const DISCONNECT: u8 = 0xFE;
    pub const SHORT_UPLOAD: u8 = 0xF4;
}

fn encode_u32(value: u32, is_be: bool, buf: &mut impl BufMut) {
    if is_be {
        buf.put_u32(value);
    } else {
        buf.put_u32_le(value);
    }
}

fn decode_u16(buf: &mut impl Buf, is_be: bool) -> u16 {
    if is_be {
        buf.get_u16()
    } else {
        buf.get_u16_le()
    }
}

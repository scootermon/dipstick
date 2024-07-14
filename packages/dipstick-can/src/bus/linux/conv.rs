use std::time::SystemTime;

use bytes::Bytes;
use dipstick_proto::can::v1::{Frame, Id};
use socketcan::{CanAnyFrame, CanFdFrame, CanFrame, EmbeddedFrame};
use tonic::{Result, Status};

pub fn to_linux_id(id: Id) -> Result<socketcan::Id> {
    if id.extended {
        socketcan::ExtendedId::new(id.id)
            .map(socketcan::Id::Extended)
            .ok_or_else(|| Status::invalid_argument("invalid can id"))
    } else {
        socketcan::StandardId::new(id.id.try_into().unwrap_or(u16::MAX))
            .map(socketcan::Id::Standard)
            .ok_or_else(|| Status::invalid_argument("invalid can id"))
    }
}

pub fn from_linux_id(id: socketcan::Id) -> Id {
    match id {
        socketcan::Id::Standard(id) => Id {
            id: id.as_raw().into(),
            extended: false,
        },
        socketcan::Id::Extended(id) => Id {
            id: id.as_raw(),
            extended: true,
        },
    }
}

pub fn to_linux_frame(frame: &Frame) -> Result<CanFrame> {
    let id = to_linux_id(frame.id.clone().unwrap_or_default())?;
    CanFrame::new(id, &frame.data).ok_or_else(|| Status::invalid_argument("frame data too long"))
}

pub fn _to_linux_fd_frame(frame: &Frame) -> Result<CanFdFrame> {
    let id = to_linux_id(frame.id.clone().unwrap_or_default())?;
    CanFdFrame::new(id, &frame.data).ok_or_else(|| Status::invalid_argument("frame data too long"))
}

pub fn from_linux_frame(frame: &CanAnyFrame, timestamp: SystemTime) -> Frame {
    match frame {
        CanAnyFrame::Normal(frame) => Frame {
            timestamp: Some(timestamp.into()),
            id: Some(from_linux_id(frame.id())),
            data: Bytes::copy_from_slice(frame.data()),
        },
        CanAnyFrame::Remote(frame) => Frame {
            timestamp: Some(timestamp.into()),
            id: Some(from_linux_id(frame.id())),
            data: Bytes::new(), // TODO,
        },
        CanAnyFrame::Error(_frame) => todo!(), // TODO
        CanAnyFrame::Fd(frame) => Frame {
            timestamp: Some(timestamp.into()),
            id: Some(from_linux_id(frame.id())),
            data: Bytes::copy_from_slice(frame.data()),
        },
    }
}

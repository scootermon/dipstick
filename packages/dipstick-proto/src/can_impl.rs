use bytes::Bytes;

use crate::can::v1::{Frame, Id, Protocol};
use crate::core::v1::Direction;

impl Frame {
    pub const fn builder() -> FrameBuilder {
        FrameBuilder::new()
    }
}

pub struct FrameBuilder {
    inner: Frame,
}

impl FrameBuilder {
    pub const fn new() -> Self {
        Self {
            inner: Frame {
                direction: Direction::Unspecified as _,
                protocol: Protocol::Unspecified as _,
                id: None,
                length: 0,
                data: Bytes::new(),
                is_remote: false,
                is_error: false,
                received_at: None,
            },
        }
    }

    pub fn build(self) -> Frame {
        self.inner
    }

    const fn direction(mut self, direction: Direction) -> Self {
        self.inner.direction = direction as _;
        self
    }

    pub const fn direction_tx(self) -> Self {
        self.direction(Direction::Transmit)
    }

    pub const fn direction_rx(self) -> Self {
        self.direction(Direction::Receive)
    }

    pub const fn id(mut self, id: Id) -> Self {
        self.inner.id = Some(id);
        self
    }

    pub fn data(mut self, data: Bytes) -> Self {
        self.inner.data = data;
        self
    }
}

impl Default for FrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Id {
    pub const fn new_standard(value: u16) -> Self {
        Self {
            value: value as _,
            is_extended: false,
        }
    }

    pub const fn new_extended(value: u32) -> Self {
        Self {
            value,
            is_extended: true,
        }
    }
}

impl Copy for Id {}

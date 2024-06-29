use crate::core::v1::Direction;
use crate::wkt::Timestamp;
use crate::xcp::v1::{CtoCommand, Frame, FrameContent};

impl Frame {
    pub const fn builder() -> FrameBuilder {
        FrameBuilder::new()
    }

    pub fn timestamp(&self) -> Option<&Timestamp> {
        match self.frame_content.as_ref()? {
            FrameContent::Command(content) => content.timestamp.as_ref(),
            FrameContent::Response(content) => content.timestamp.as_ref(),
        }
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
                frame_content: None,
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

    fn content(mut self, content: FrameContent) -> Self {
        self.inner.frame_content = Some(content);
        self
    }

    pub fn content_command(self, command: CtoCommand) -> Self {
        self.content(FrameContent::Command(command))
    }
}

impl Default for FrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

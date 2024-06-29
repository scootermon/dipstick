use std::marker::PhantomData;

use bytes::BytesMut;
use dipstick_proto::Message;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct History<M: Message> {
    buf: BytesMut,
    file: File,
    _phantom: PhantomData<M>,
}

impl<M: Message> History<M> {
    async fn write(&mut self, msg: M) -> anyhow::Result<()> {
        self.buf.clear();
        msg.encode_length_delimited(&mut self.buf)?;
        self.file.write(&self.buf).await?;
        Ok(())
    }
}

use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;
use crate::message::ChatMessage;

pub struct Sender {
    sock: OwnedWriteHalf,
}
impl Sender {
    pub fn new(sock: OwnedWriteHalf) -> Self {
        Self { sock }
    }

    pub async fn send(&mut self, msg: ChatMessage) -> crate::Result<()> {
        let bytes = rmp_serde::to_vec(&msg)?;
        self.send_bytes(&bytes).await
    }

    pub async fn send_bytes(&mut self, bytes: &[u8]) -> crate::Result<()> {
        self.sock.write_all(&bytes).await?;
        return Ok(());
    }
}

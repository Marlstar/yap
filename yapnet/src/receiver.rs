use tokio::net::tcp::OwnedReadHalf;
use tokio::io::AsyncReadExt;
use crate::message::ChatMessage;

pub struct Receiver {
    sock: OwnedReadHalf,
}
impl Receiver {
    pub fn new(sock: OwnedReadHalf) -> Self {
        Self { sock }
    }

    pub async fn recv_chat_message(&mut self) -> crate::Result<ChatMessage> {
        let bytes = self.recv_bytes().await?;
        let msg = rmp_serde::from_slice(&bytes)?;
        return Ok(msg);
    }

    pub async fn recv_bytes(&mut self) -> Result<Vec<u8>, tokio::io::Error> {
        let expected = self.sock.read_u32().await? as usize;
        let mut buf = vec![0u8; expected];
        self.sock.read_exact(&mut buf).await?;
        return Ok(buf);
    }
}

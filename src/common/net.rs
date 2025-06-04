use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt,AsyncReadExt};

pub async fn read_message<M: serde::de::DeserializeOwned>(sock: &mut TcpStream) -> Result<M, tokio::io::Error> {
    let bytes = read_bytes(sock).await?;
    let msg = rmp_serde::from_slice::<M>(&bytes).expect("failed to deserialise message");
    return Ok(msg);
}

pub async fn read_bytes(sock: &mut TcpStream) -> tokio::io::Result<Vec<u8>> {
    let byte_count = sock.read_u32().await? as usize;
    return read_bytes_noheader(sock, byte_count).await;
}

pub async fn read_bytes_noheader(sock: &mut TcpStream, byte_count: usize) -> tokio::io::Result<Vec<u8>> {
    let mut buf = vec![0; byte_count];
    sock.read_exact(&mut buf).await?;
    return Ok(buf);
}

pub async fn write_message<M: serde::Serialize>(sock: &mut TcpStream, msg: M) -> tokio::io::Result<()> {
    let bytes = rmp_serde::to_vec(&msg).expect("failed to serialise message");
    write_bytes(sock, &bytes).await?;

    return Ok(());
}
pub async fn write_bytes(sock: &mut TcpStream, bytes: &[u8]) -> tokio::io::Result<()> {
    sock.write_u32(bytes.len() as u32).await?;
    write_bytes_noheader(sock, bytes).await
}

pub async fn write_bytes_noheader(sock: &mut TcpStream, bytes: &[u8]) -> tokio::io::Result<()> {
    sock.write_all(bytes).await
}

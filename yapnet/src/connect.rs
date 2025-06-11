use tokio::net::TcpStream;
use super::{Sender, Receiver};

pub async fn connect(addr: &str) -> Result<(Sender, Receiver), tokio::io::Error> {
    let sock = TcpStream::connect(addr).await?;
    let (read, write) = sock.into_split();

    let sender = Sender::new(write);
    let receiver = Receiver::new(read);

    return Ok((sender, receiver));
}

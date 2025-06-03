use tokio::net::TcpStream;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct ClientHandler {
    pub uuid: Uuid,
    sock: TcpStream,
    addr: SocketAddr,
}
impl ClientHandler {
    pub fn new(sock: TcpStream, addr: SocketAddr) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            sock,
            addr,
        }
    }
}

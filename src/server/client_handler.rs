use tokio::io::Interest;
use tokio::net::TcpStream;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct ClientHandler {
    pub uuid: Uuid,
    sock: TcpStream,
    pub addr: SocketAddr,
}
impl ClientHandler {
    pub fn new(sock: TcpStream, addr: SocketAddr) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            sock,
            addr,
        }
    }

    pub async fn heartbeat(&self) -> bool {
        match self.sock.ready(Interest::READABLE | Interest::WRITABLE).await {
            Ok(ready) => {
                // Honestly not sure why you would invert this
                // but that's what it needs ¯\_(ツ)_/¯
                return !(ready.is_readable() && ready.is_writable());
            },
            Err(_) => return false,
        };
    }

    pub async fn handle_remote_disconnect(&self) {
        // For future if I need to do anything in the handler when client disconnects
    }
}

use std::net::SocketAddr;
use tokio::io::Interest;
use tokio::net::TcpStream;

pub struct Client {
    sock: TcpStream,
}
impl Client { // Construction
    pub async fn new(server_addr: SocketAddr) -> Self {
        // TODO: error handling
        Self {
            sock: Self::connect(server_addr).await.unwrap(),
        }
    }

    async fn connect(addr: SocketAddr) -> Result<TcpStream, tokio::io::Error> {
        let s = TcpStream::connect(addr).await;

        match &s {
            Ok(_) => println!("[conn] connected to server {addr:?} successfully"),
            Err(e) => println!("[con] failed to connect to server {addr:?} => {e}")
        };

        return s;
    }
}
impl Client { // Functionality
    pub async fn run(&mut self) {
        println!("[main] client running");
        
        // TODO: remove this allow once I add more functionality
        #[allow(clippy::never_loop)]
        loop {
            tokio::select! {
                _ = self.watch_for_server_disconnect() => { println!("[conn] server closed"); break; },
            }
        }
        println!("shutting down");
    }

    async fn watch_for_server_disconnect(&self) {
        loop {
            if !self.check_server_status().await { return; }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    async fn check_server_status(&self) -> bool {
        match self.sock.ready(Interest::READABLE | Interest::WRITABLE).await {
            Ok(ready) => {
                // Honestly not sure why you would invert this
                // but that's what it needs ¯\_(ツ)_/¯
                return !(ready.is_readable() && ready.is_writable());
            },
            Err(_) => return false,
        };
    }
}

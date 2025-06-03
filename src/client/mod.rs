use std::net::SocketAddr;

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
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
    }
}

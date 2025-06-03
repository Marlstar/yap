use client_handler::ClientHandler;
use hashbrown::HashMap;
use uuid::Uuid;
use tokio::net::{TcpListener, TcpStream};
use std::net::SocketAddr;

pub mod client_handler;

pub const PORT: u16 = 31415;


pub struct Server {
    clients: HashMap<Uuid, ClientHandler>,

    // Network stuff
    sock: TcpListener,
}
impl Server { // Construction
    pub async fn new() -> Self {
        // TODO: error handling
        Self {
            clients: HashMap::new(),
            
            sock: Self::bind_sock(PORT).await.unwrap(),
        }
    }

    async fn bind_sock(port: u16) -> Result<TcpListener, tokio::io::Error> {
        return TcpListener::bind(format!("0.0.0.0:{port}")).await;
    }
}
impl Server { // Functionality
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                (client, addr) = self.accept_client() => self.onboard_client(client, addr).await,
            };
        }
    }

    async fn accept_client(&mut self) -> (TcpStream, SocketAddr) {
        // TODO: error handling
        return self.sock.accept().await.unwrap();
    }

    async fn onboard_client(&mut self, client: TcpStream, addr: SocketAddr) {
        println!("[clients] onboarding client {addr:?}");
        let handler = ClientHandler::new(client, addr);

        self.clients.insert(handler.uuid, handler);
    }
}

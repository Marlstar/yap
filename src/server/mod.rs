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
        println!("[main] server running");
        loop {
            tokio::select! {
                (client, addr) = self.accept_client() => self.onboard_client(client, addr).await,
                client = self.check_client_heartbeats() => self.handle_client_disconnect(client).await,
            };
        }
    }

    async fn accept_client(&self) -> (TcpStream, SocketAddr) {
        // TODO: error handling
        return self.sock.accept().await.unwrap();
    }

    async fn onboard_client(&mut self, client: TcpStream, addr: SocketAddr) {
        println!("[clients] onboarding client {addr:?}");
        let handler = ClientHandler::new(client, addr);

        self.clients.insert(handler.uuid, handler);
        dbg!(self.receive_messages().await);
    }

    async fn handle_client_disconnect(&mut self, client: Uuid) {
        let client = self.clients.remove(&client).expect("tried to remove a nonexistent client");
        client.handle_remote_disconnect().await;
    }

    // TODO: make this the responsibility of the client handlers rather than the server itself
    async fn check_client_heartbeats(&self) -> Uuid {
        loop {
            for client in self.clients.values() {
                if !client.heartbeat().await {
                    println!("[{:?}] disconnected", client.addr);
                    return client.uuid;
                };
            }

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    async fn receive_messages(&mut self) -> (Uuid, crate::client::Message) {
        loop {
            for client in self.clients.values_mut() {
                if let Some(msg) = client.receive_message().await {
                    return (client.uuid, msg);
                }
            }
        }
    }
}

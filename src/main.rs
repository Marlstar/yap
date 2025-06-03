use std::net::ToSocketAddrs;
use yap::server::Server;
use yap::client::Client;

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args[1] == "server" {
        Server::new().await.run().await;
    } else {
        let addr = args[1].to_socket_addrs()
            .expect("expected a valid server address")
            .next()
            .expect("expected a valid server address");

        Client::new(addr).await.run().await;
    }
}

use yap::server::Server;

#[tokio::main]
async fn main() {
    Server::new().await.run().await;
}

use tokio::net::tcp::OwnedWriteHalf;

pub struct Sender {
    sock: OwnedWriteHalf,
}
impl Sender {
    pub fn new(sock: OwnedWriteHalf) -> Self {
        Self { sock }
    }
}

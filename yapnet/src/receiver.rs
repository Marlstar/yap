use tokio::net::tcp::OwnedReadHalf;

pub struct Receiver {
    sock: OwnedReadHalf,
}
impl Receiver {
    pub fn new(sock: OwnedReadHalf) -> Self {
        Self { sock }
    }
}

#![allow(clippy::needless_return)]

use sender::Sender;
use receiver::Receiver;

pub mod sender;
pub mod receiver;
pub mod message;

pub mod error;
pub use error::Error;
pub use error::Result;

mod connect;
pub use connect::connect;

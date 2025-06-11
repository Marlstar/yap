#[derive(Debug)]
pub enum Error {
    IO(tokio::io::Error),
    Serialise(rmp_serde::encode::Error),
    Deserialise(rmp_serde::decode::Error),
}

macro_rules! e {
    ($from:path, $to:ident) => {
        impl From<$from> for Error { fn from(value: $from) -> Self { Self::$to(value) } }
    };
    ($from:path, $to:ident, $fn:expr) => {
        impl From<$from> for Error { fn from(value: $from) -> Self { Self::$to($fn(value)) } }
    };
}

e!(tokio::io::Error, IO);
e!(rmp_serde::encode::Error, Serialise);
e!(rmp_serde::decode::Error, Deserialise);

pub type Result<T> = std::result::Result<T, Error>;

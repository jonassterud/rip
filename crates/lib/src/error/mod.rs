#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Poison(#[from] std::sync::PoisonError<()>),
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("bencode error: {0}")]
    Bencode(String),
    #[error("torrent error: {0}")]
    Torrent(String),
    #[error("unknown error")]
    Unknown,
}

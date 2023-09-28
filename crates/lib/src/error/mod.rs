#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Poison(#[from] std::sync::PoisonError<()>),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error("unknown error")]
    Unknown,
}

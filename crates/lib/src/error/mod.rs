// TODO: Add backtrace after this is merged: https://github.com/rust-lang/rust/issues/99301

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Bcode error.
    #[error("bencode error: {0}")]
    Bencode(String),
    /// Torrent error.
    #[error("torrent error: {0}")]
    Torrent(String),
    /// Tracker error.
    #[error("tracker error: {0}")]
    Tracker(String),
    /// Agent error.
    #[error("agent error: {0}")]
    Agent(String),
    /// Unexpected error.
    #[error("unexpected error: {0}")]
    Unexpected(String),
    /// [`std::io::Error`]
    #[error("I/O error: {source:?}")]
    _Io {
        #[from]
        source: std::io::Error,
    },
    /// [`std::sync::PoisonError`]
    #[error("thread paniced, and caused a poison error: {source:?}")]
    _Poision {
        #[from]
        source: std::sync::PoisonError<()>,
    },
    /// [`tokio::task::JoinError`]
    #[error("task failed to execute to completion: {source:?}")]
    _Join {
        #[from]
        source: tokio::task::JoinError,
    },
    /// [`std::string::FromUtf8Error`]
    #[error("converting byte vector to String failed: {source:?}")]
    _FromUtf8 {
        #[from]
        source: std::string::FromUtf8Error,
    },
    /// [`reqwest::Error`]
    #[error("processing a request failed: {source:?}")]
    _Reqwest {
        #[from]
        source: reqwest::Error,
    },
}

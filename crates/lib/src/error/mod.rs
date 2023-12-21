use crate::prelude::*;
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
    /// Peer error.
    #[error("peer error: {0}")]
    Peer(String),
    /// Agent error.
    #[error("agent error: {0}")]
    Agent(String),
    /// Helper error.
    #[error("helper error: {0}")]
    Helper(String),
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
    /// [`std::array::TryFromSliceError`]
    #[error("from slice to integer: {source:?}")]
    _TryFromSliceError {
        #[from]
        source: std::array::TryFromSliceError,
    },
    /// [`reqwest::Error`]
    #[error("processing a request failed: {source:?}")]
    _Reqwest {
        #[from]
        source: reqwest::Error,
    },
    /// [`std::sync::mpsc::SendError`]
    #[error("sending/receiving a message failed: {source:?}")]
    _StdSend {
        #[from]
        source: std::sync::mpsc::SendError<PeerMessage>,
    },
    /// [`tokio::sync::mpsc::error::SendError`]
    #[error("sending/receiving a message failed: {source:?}")]
    _TokioSendPeerMessage {
        #[from]
        source: tokio::sync::mpsc::error::SendError<PeerMessage>,
    },
    /// [`tokio::sync::mpsc::error::SendError`]
    #[error("sending/receiving a message failed: {source:?}")]
    _TokioSendPiece {
        #[from]
        source: tokio::sync::mpsc::error::SendError<(u32, u32, Vec<u8>)>,
    },
}

// https://wiki.theory.org/BitTorrentSpecification

mod info;
mod parse;
mod tracker;

use super::agent::traits::Download;
use super::error::Error;
use crate::prelude::*;
use info::TorrentInfo;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;

pub use tracker::Tracker;

/// Torrent.
#[derive(Debug, Clone)]
pub struct Torrent {
    /// Torrent info.
    pub info: TorrentInfo,
    /// Tracker URL.
    pub announce: String,
    /// Optional announce list.
    pub announce_list: Option<Vec<Vec<String>>>,
    /// Optional creation date.
    pub creation_date: Option<usize>,
    /// Optional comment.
    pub comment: Option<String>,
    /// Optional creator name.
    pub created_by: Option<String>,
    /// Optional encoding type.
    pub encoding: Option<String>,

    /// SHA1 hash of info dictionary.
    info_hash: Vec<u8>,
    /// Download buffer.
    buffer: Vec<u8>,
    /// Total amount uploaded.
    _uploaded: usize,
}

impl Torrent {
    /// Create [`Torrent`] from Bencoded bytes.
    pub fn from_bcode(contents: &[u8]) -> Result<Self, Error> {
        Torrent::parse(contents)
    }

    /// Read a pending [`Torrent`] download process stored on disk.
    pub fn from_disk() -> Result<Self, Error> {
        todo!()
    }

    /// Get `info_hash`.
    pub fn get_hash(&self) -> &[u8] {
        self.info_hash.as_slice()
    }
}

impl Download for Torrent {
    type Error = Error;

    fn initiate(
        &self,
        agent: &Agent,
        out: &Path,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>>>> {
        let tracker_request = Tracker::create_request(&self, agent);
        Box::pin(async move {
            let tracker_response = tracker_request?.send().await?;
            todo!("continue...");

            Ok(())
        })
    }

    fn get_uploaded(&self) -> usize {
        self._uploaded
    }

    fn get_downloaded(&self) -> usize {
        self.buffer.len()
    }

    fn get_left(&self) -> usize {
        // TODO:
        // From BEP-03:
        // "Note that this can't be computed from downloaded
        // and the file length since it might be a resume, and
        // there's a chance that some of the downloaded data failed
        // an integrity check and had to be re-downloaded."

        // TODO: I think this is right (minus "TODO" above)
        (self.info.piece_length * (self.info.pieces.len() / 20)) - self.get_downloaded()
    }
}

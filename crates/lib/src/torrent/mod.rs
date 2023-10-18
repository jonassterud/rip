// https://wiki.theory.org/BitTorrentSpecification

mod info;
mod parse;
mod tracker;

use super::agent::traits::Download;
use super::error::Error;
use crate::prelude::*;
use std::future::Future;
use std::pin::Pin;
use info::TorrentInfo;
use std::path::Path;

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
}

impl Torrent {
    /// Create [`Torrent`] from bencoded bytes.
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        Torrent::parse(contents)
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
        0
        //todo!()
    }

    fn get_downloaded(&self) -> usize {
        0
        //todo!()
    }

    fn get_left(&self) -> usize {
        99999999
        //todo!()
    }
}

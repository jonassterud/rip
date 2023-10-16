use crate::prelude::*;
use crate::error::Error;
use rand::distributions::{Alphanumeric, DistString};

/// Tracker GET request.
pub struct TrackerRequest {
    /// Torrent info hash.
    pub info_hash: Vec<u8>,
    /// Random peer id.
    pub peer_id: String,
    /// Optional peer ip.
    pub ip: Option<String>,
    /// Port peer is listening at.
    pub port: u16,
    /// Total amount uploaded.
    pub uploaded: usize,
    /// Total amount downloaded.
    pub downloaded: usize,
    /// Total amount left.
    pub left: usize,
    /// Optional status.
    pub event: Option<String>,
}

impl TrackerRequest {
    /// Create a [`TrackerRequest`] from a [`Torrent`] and its [`Agent`].
    pub fn from_torrent(torrent: &Torrent, agent: &Agent) -> Result<Self, Error> {
        let file = agent.get_file(torrent.get_hash())?;

        Ok(Self {
            info_hash: torrent.get_hash().to_vec(),
            peer_id: Alphanumeric.sample_string(&mut rand::thread_rng(), 20),
            ip: None,
            port: agent.get_port(),
            uploaded: file.get_uploaded(),
            downloaded: file.get_downloaded(),
            left: file.get_left(),
            event: None,
        })
    }

    /// Get [`TrackerRequest`] as URL parameters.
    pub fn as_url_params(&self) -> String {
        todo!()
    }
}

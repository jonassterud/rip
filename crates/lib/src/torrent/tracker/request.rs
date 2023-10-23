use super::TrackerResponse;
use crate::error::Error;
use crate::prelude::*;
use rand::distributions::{Alphanumeric, DistString};

/// Tracker GET request.
#[derive(Debug)]
pub struct TrackerRequest {
    /// Tracker URL.
    pub announce: String,
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
    pub fn with(torrent: &Torrent, agent: &Agent) -> Result<Self, Error> {
        let file = agent.get_file(&torrent.get_hash())?;

        Ok(Self {
            announce: torrent.announce.clone(),
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

    /// Send [`TrackerRequest`] and wait for [`TrackerResponse`].
    pub async fn send(&self) -> Result<TrackerResponse, Error> {
        let final_url = format!(
            "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}",
            self.announce,
            urlencoding::encode_binary(&self.info_hash),
            urlencoding::encode(&self.peer_id),
            urlencoding::encode(&self.port.to_string()),
            urlencoding::encode(&self.uploaded.to_string()),
            urlencoding::encode(&self.downloaded.to_string()),
            urlencoding::encode(&self.left.to_string()),
            //urlencoding::encode(self.event.as_ref().unwrap_or(&"".to_string())),
        );
        println!("{:?}", final_url);
        let bytes = reqwest::get(final_url).await?.bytes().await?.to_vec();

        TrackerResponse::from_bcode(&bytes)
    }
}

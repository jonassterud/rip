// https://wiki.theory.org/BitTorrentSpecification

mod info;
mod parse;

use super::agent::traits::Download;
use super::error::Error;
use info::TorrentInfo;
use std::path::Path;

pub struct Torrent {
    pub info: TorrentInfo,
    pub announce: String,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub creation_date: Option<usize>,
    pub comment: Option<String>,
    pub created_by: Option<String>,
    pub encoding: Option<String>,
}

impl Torrent {
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        Torrent::parse(contents)
    }
}

impl Download for Torrent {
    type Error = Error;

    fn download(&self, out: &Path) -> Result<(), Self::Error> {
        todo!()
    }
}

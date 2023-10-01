// https://wiki.theory.org/BitTorrentSpecification

mod info;
mod parse;

use super::agent::traits::Download;
use super::error::Error;
use info::TorrentInfo;
use std::path::Path;

#[derive(Debug)]
pub struct Torrent {
    info: TorrentInfo,
    announce: String,
    announce_list: Option<Vec<Vec<String>>>,
    creation_date: Option<usize>,
    comment: Option<String>,
    created_by: Option<String>,
    encoding: Option<String>,
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

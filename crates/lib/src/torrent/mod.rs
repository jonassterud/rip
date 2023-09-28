mod parse;

use super::agent::traits::Download;
use super::error::Error;
use std::path::Path;

pub struct Torrent {
    pub announce: String,
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

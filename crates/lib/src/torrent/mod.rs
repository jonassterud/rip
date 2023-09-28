use std::path::Path;

use super::agent::traits::Download;
use super::error::Error;

pub struct Torrent {
    
}

impl Torrent {
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

impl Download for Torrent {
    type E = Error;

    fn download(&self, out: &Path) -> Result<(), Self::E> {
        todo!()
    }
}

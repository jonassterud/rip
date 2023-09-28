use super::error::Error;
use super::torrent::Torrent;
use std::path::Path;

pub struct Agent {
    
}

impl Agent {
    pub fn new() -> Result<Self, Error> {
        todo!()
    }

    pub fn add_torrent(&self, torrent: Torrent) -> Result<(), Error> {
        todo!()
    }

    pub async fn download(&self, out: &Path) -> Result<(), Error> {
        todo!()
    }
}

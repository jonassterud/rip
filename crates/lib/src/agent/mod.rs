pub mod traits;

use self::traits::Download;
use super::error::Error;
use super::torrent::Torrent;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;

pub struct Agent {
    files: Vec<Box<dyn Download<Error = Error>>>,
}

impl Agent {
    /// Create a new `Agent`.
    pub fn new() -> Result<Self, Error> {
        Ok(Self { files: Vec::new() })
    }

    /// Read and parse torrents from a list of file paths.
    pub async fn add_torrents(&mut self, paths: Vec<PathBuf>) -> Result<(), Error> {
        let mut set = JoinSet::new();

        for path in paths {
            set.spawn(tokio::spawn(async move {
                let contents = tokio::fs::read(path).await?;
                let torrent = Torrent::from_bytes(&contents)?;

                Ok::<Torrent, Error>(torrent)
            }));
        }

        while let Some(res) = set.join_next().await {
            self.files.push(Box::new(res???));
        }

        Ok(())
    }

    /// Start a download process for all pending files.
    pub async fn download(&self, out: &Path) -> Result<(), Error> {
        for file in &self.files {
            file.download(out)?;
        }

        Ok(())
    }
}

pub mod traits;

use self::traits::Download;
use super::error::Error;
use super::torrent::Torrent;
use futures::future::try_join_all;
use futures::stream::FuturesUnordered;
use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use tokio::task::JoinSet;

/// Agent, which handles download process.
pub struct Agent {
    files: HashMap<Vec<u8>, Box<dyn Download<Error = Error>>>,
    futures: FuturesUnordered<Pin<Box<dyn Future<Output = Result<(), Error>>>>>,
}

impl Agent {
    /// Create a new `Agent`.
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            files: HashMap::new(),
            futures: FuturesUnordered::new(),
        })
    }

    /// Get file with `hash`.
    pub fn get_file(&self, hash: &[u8]) -> Result<&Box<dyn Download<Error = Error>>, Error> {
        self.files
            .get(hash)
            .ok_or_else(|| Error::Agent(format!("file not found")))
    }

    /// Get IP port.
    pub fn get_port(&self) -> u16 {
        6881
        //todo!()
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
            let torrent = res???;
            let hash = torrent.get_hash().to_owned();
            self.files.insert(hash, Box::new(torrent));
        }

        Ok(())
    }

    /// Start a download process for all pending files.
    pub async fn download(self, out: &Path) -> Result<(), Error> {
        for (_, file) in &self.files {
            self.futures.push(Box::pin(file.initiate(&self, out)));
        }

        try_join_all(self.futures.into_iter()).await?;

        Ok(())
    }
}

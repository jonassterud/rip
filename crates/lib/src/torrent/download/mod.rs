use crate::agent::traits::Download;
use crate::error::Error;
use crate::prelude::*;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use futures::future::try_join_all;
use tokio::task::JoinHandle;
use rand::distributions::{Distribution, Alphanumeric};

impl Download for Torrent {
    type Error = Error;

    fn initiate(
        &self,
        agent: &Agent,
        out: &Path,
    ) -> JoinHandle<Result<(), Error>> {
        let hash = self.get_hash().to_vec();
        let id = Alphanumeric.sample_iter(&mut rand::thread_rng()).take(20).collect::<Vec<u8>>();
        let tracker_request = Tracker::create_request(&self, agent, &id);

        tokio::spawn(async move {
            let tracker_response = tracker_request?.send().await?;
            let mut tasks: Vec<JoinHandle<Result<(), Error>>> = Vec::new();
        
            for mut peer in tracker_response.peers {
                peer.connect(10, &hash, &id)?;

                tasks.push(tokio::spawn(async move {
                    peer.join().await?;

                    Ok(())
                }));
    
                //todo!("continue...");
            }

            try_join_all(tasks).await?;
    
            Ok(())
        })
    }

    fn get_uploaded(&self) -> usize {
        self._uploaded
    }

    fn get_downloaded(&self) -> usize {
        self.buffer.len()
    }

    fn get_left(&self) -> usize {
        // TODO:
        // From BEP-03:
        // "Note that this can't be computed from downloaded
        // and the file length since it might be a resume, and
        // there's a chance that some of the downloaded data failed
        // an integrity check and had to be re-downloaded."

        // TODO: I think this is right (minus "TODO" above)
        (self.info.piece_length * (self.info.pieces.len() / 20)) - self.get_downloaded()
    }
}

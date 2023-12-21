use crate::agent::traits::Download;
use crate::error::Error;
use crate::prelude::*;
use futures::future::try_join_all;
use rand::distributions::{Alphanumeric, Distribution};
use std::path::Path;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::task::JoinHandle;

impl Download for Torrent {
    type Error = Error;

    fn initiate(&self, agent: &Agent, out: &Path) -> JoinHandle<Result<(), Error>> {
        let hash = self.get_hash().to_vec();
        let id = Alphanumeric
            .sample_iter(&mut rand::thread_rng())
            .take(20)
            .collect::<Vec<u8>>();
        let tracker_request = Tracker::create_request(&self, agent, &id);

        let download_buffer = self.buffer.clone();
        let piece_length = self.info.piece_length;
        tokio::spawn(async move {
            let tracker_response = tracker_request?.send().await?;
            let mut tasks: Vec<JoinHandle<Result<(), Error>>> = Vec::new();

            for mut peer in tracker_response.peers {
                if peer.connect(10, &hash, &id).await.is_ok() {
                    let (piece_sender, mut piece_receiver) = tokio_mpsc::unbounded_channel();
                    peer.handle_messages(piece_sender).await?;

                    let mut peer = peer.clone();
                    tasks.push(tokio::spawn(async move {
                        peer.join().await?;

                        Ok(())
                    }));

                    let download_buffer = download_buffer.clone();
                    tasks.push(tokio::spawn(async move {
                        while let Some(piece) = piece_receiver.recv().await {
                            let start_index = (piece.0 as usize * piece_length) + piece.1 as usize;
                            let end_index = start_index + piece.2.len();

                            download_buffer
                                .lock()
                                .await
                                .splice(start_index..end_index, piece.2);
                        }

                        Ok(())
                    }));
                }

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
        0 // TODO
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

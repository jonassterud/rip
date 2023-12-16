mod request;
mod response;

use crate::error::Error;
use crate::prelude::*;
use request::TrackerRequest;
use response::TrackerResponse;

/// A torrent tracker.
#[derive(Debug)]
pub struct Tracker {}

impl Tracker {
    /// Create a [`TrackerRequest`] for a `torrent` and its `agent`.
    pub fn create_request(
        torrent: &Torrent,
        agent: &Agent,
        id: &[u8],
    ) -> Result<TrackerRequest, Error> {
        TrackerRequest::with(torrent, agent, id)
    }
}

mod request;
mod response;

use crate::prelude::*;
use crate::error::Error;
use request::TrackerRequest;
use response::TrackerResponse;



/// A torrent tracker.
#[derive(Debug)]
pub struct Tracker {
    hash: Vec<u8>,
    request: Option<TrackerRequest>,
    response: Option<TrackerResponse>
}

impl Tracker {
    pub fn with(hash: &[u8]) -> Self {
        todo!()
    }
}

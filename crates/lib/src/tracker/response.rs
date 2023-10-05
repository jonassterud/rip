use crate::error::Error;
use crate::prelude::*;
use std::collections::BTreeMap;

pub struct TrackerResponse {
    interval: usize,
    peers: Vec<Peer>,
}

impl TrackerResponse {
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        let dict = Value::from_bytes(contents)?.try_as::<Dictionary>()?;

        if let Ok(failure_reason) = dict.try_get_as::<ByteString>("failure reason") {
            return Err(Error::Tracker(String::from_utf8(failure_reason.0)?));
        }

        let interval = dict.try_get_as::<Integer>("interval")?.0 as usize;
        let peers = dict
            .try_get("peers")?
            .clone()
            .as_list_of::<Dictionary>()?
            .iter()
            .map(Peer::from_dictionary)
            .collect::<Result<Vec<Peer>, Error>>()?;

        Ok(Self { interval, peers })
    }
}

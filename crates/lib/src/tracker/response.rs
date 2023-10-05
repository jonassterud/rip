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

fn get(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> Result<Value, Error> {
    Ok(dictionary
        .get(k.as_bytes())
        .ok_or_else(|| Error::Torrent(format!("missing key {k}")))?
        .clone())
}

fn get_opt(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> Option<Value> {
    dictionary.get(k.as_bytes()).cloned()
}

fn has(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> bool {
    dictionary.contains_key(k.as_bytes())
}

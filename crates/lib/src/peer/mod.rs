use crate::error::Error;
use crate::prelude::*;
use std::collections::BTreeMap;

pub struct Peer {
    id: Vec<u8>,
    ip: Vec<u8>,
    port: Vec<u8>,
}

impl Peer {
    pub fn from_dict(dict: &BTreeMap<Vec<u8>, Value>) -> Result<Self, Error> {
        let id = get(dict, "peer id")?.as_byte_string()?;
        let ip = get(dict, "ip")?.as_byte_string()?;
        let port = get(dict, "port")?.as_byte_string()?;

        Ok(Self { id, ip, port })
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

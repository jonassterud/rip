use super::*;
use crate::bencode::Value;
use std::collections::BTreeMap;

impl Torrent {
    pub fn parse(contents: &[u8]) -> Result<Self, Error> {
        let dictionary = Value::from_bytes(contents)?.as_dictionary()?;
        let info = get(&dictionary, "info")?.as_dictionary()?;
        let announce = String::from_utf8(get(&dictionary, "announce")?.as_byte_string()?)?;
        let announce_list = None;
        let creation_date = None;
        let comment = None;
        let created_by = None;
        let encoding = None;

        Ok(Torrent {
            info: TorrentInfo::from_dictionary(info)?,
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,
        })
    }
}

fn get(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> Result<Value, Error> {
    Ok(dictionary
        .get(k.as_bytes())
        .ok_or_else(|| Error::Torrent(format!("missing key {k}")))?
        .clone())
}

use super::*;
use crate::prelude::*;
use std::collections::BTreeMap;

impl Torrent {
    pub fn parse(contents: &[u8]) -> Result<Self, Error> {
        let dictionary = Value::from_bytes(contents)?.try_as::<Dictionary>()?;
        let info = dictionary.try_get_as::<Dictionary>("info")?;
        let announce = String::from_utf8(dictionary.try_get_as::<ByteString>("announce")?.0)?;
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

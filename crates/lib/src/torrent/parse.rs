use super::*;
use crate::bencode::Value;
use std::collections::BTreeMap;

impl Torrent {
    pub fn parse(contents: &[u8]) -> Result<Self, Error> {
        let dictionary = Value::from_bytes(contents)?.try_as_dictionary()?;
        let announce = String::from_utf8(get(&dictionary, "announce")?.try_as_byte_string()?)?;
        let info = get(&dictionary, "info")?.try_as_dictionary()?;

        Ok(Torrent { announce })
    }
}

fn get(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> Result<Value, Error> {
    Ok(dictionary
        .get(k.as_bytes())
        .ok_or_else(|| Error::Torrent("missing key".into()))?
        .clone())
}

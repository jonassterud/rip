use crate::bencode::Value;
use crate::error::Error;
use std::collections::BTreeMap;

pub struct TorrentInfo {
    pub files: Vec<File>,
    pub name: Vec<u8>,
    pub piece_length: usize,
    pub pieces: Vec<u8>,
    pub private: Option<bool>,
    is_single_file: bool,
}

pub struct File {
    pub length: usize,
    pub path: Vec<Vec<u8>>,
    pub md5sum: Option<Vec<u8>>,
}

impl TorrentInfo {
    pub fn from_dictionary(info: BTreeMap<Vec<u8>, Value>) -> Result<Self, Error> {
        let mut files = Vec::new();
        let name = get(&info, "name")?.try_as_byte_string()?;
        let piece_length = get(&info, "piece length")?.try_as_integer()? as usize;
        let pieces = get(&info, "pieces")?.try_as_byte_string()?;
        let private = get_opt(&info, "private").map(|v| v.try_as_integer().ok() != Some(0));
        let is_single_file = has(&info, "files");

        if is_single_file {
            let length = get(&info, "length")?.try_as_integer()? as usize;
            let md5sum = get_opt(&info, "md5sum")
                .map(|s| s.try_as_byte_string())
                .transpose()?;

            files.push(File {
                length,
                path: Vec::new(),
                md5sum,
            });
        } else {
            let mut tmp = get(&info, "files")?
                .try_as_list()?
                .into_iter()
                .map(|v| {
                    let d = v.try_as_dictionary()?;
                    let length = get(&d, "length")?.try_as_integer()? as usize;
                    let path = get(&d, "path")?.try_as_list_of_byte_strings()?;
                    let md5sum = get_opt(&d, "md5sum")
                        .map(|s| s.try_as_byte_string())
                        .transpose()?;

                    Ok::<File, Error>(File {
                        length,
                        path,
                        md5sum,
                    })
                })
                .collect::<Result<Vec<File>, Error>>()?;

            files.append(&mut tmp);
        }

        Ok(Self {
            files,
            name,
            piece_length,
            pieces,
            private,
            is_single_file,
        })
    }
}

fn get(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> Result<Value, Error> {
    Ok(dictionary
        .get(k.as_bytes())
        .ok_or_else(|| Error::Torrent("missing key".into()))?
        .clone())
}

fn get_opt(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> Option<Value> {
    dictionary.get(k.as_bytes()).cloned()
}

fn has(dictionary: &BTreeMap<Vec<u8>, Value>, k: &str) -> bool {
    dictionary.contains_key(k.as_bytes())
}

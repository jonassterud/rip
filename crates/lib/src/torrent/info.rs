use crate::error::Error;
use crate::prelude::*;

#[derive(Debug)]
pub struct TorrentInfo {
    files: Vec<File>,
    name: Vec<u8>,
    piece_length: usize,
    pieces: Vec<u8>,
    private: Option<bool>,
    is_single_file: bool,
}

#[derive(Debug)]
pub struct File {
    pub length: usize,
    pub path: Vec<Vec<u8>>,
    pub md5sum: Option<Vec<u8>>,
}

impl TorrentInfo {
    pub fn from_dictionary(info: Dictionary) -> Result<Self, Error> {
        let mut files = Vec::new();
        let name = info.try_get_as::<ByteString>("name")?.0;
        let piece_length = info.try_get_as::<Integer>("piece length")?.0 as usize;
        let pieces = info.try_get_as::<ByteString>("pieces")?.0;
        let private = info
            .try_get_as::<Integer>("private")
            .ok()
            .map(|v| v != Integer(0));
        let is_single_file = !info.has("files");

        if is_single_file {
            let length = info.try_get_as::<Integer>("length")?.0 as usize;
            let md5sum = info.try_get_as::<ByteString>("md5sum").ok().map(|v| v.0);

            files.push(File {
                length,
                path: Vec::new(),
                md5sum,
            });
        } else {
            let mut tmp = info
                .try_get_as::<List>("files")?
                .0
                .into_iter()
                .map(|v| {
                    let d = v.try_as::<Dictionary>()?;
                    let length = d.try_get_as::<Integer>("length")?.0 as usize;
                    let path = d
                        .try_get("path")?
                        .clone()
                        .as_list_of::<ByteString>()?
                        .into_iter()
                        .map(|v| v.0)
                        .collect();
                    let md5sum = d.try_get_as::<ByteString>("md5sum").ok().map(|v| v.0);

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

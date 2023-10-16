use crate::error::Error;
use crate::prelude::*;

/// Torrent info.
#[derive(Debug)]
pub struct TorrentInfo {
    /// List of files.
    pub files: Vec<File>,
    /// Suggested name to save the file (or directory if multipile files).
    pub name: Vec<u8>,
    /// Number of bytes in each piece the file is split into.
    pub piece_length: usize,
    /// All SHA1 hashes of pieces in order (i.e. multipiles of 20)
    pub pieces: Vec<u8>,
    /// Optional private flag.
    pub private: Option<bool>,
    /// Whether the [`Torrent`] is for a single or multipile files.
    pub is_single_file: bool,
}

/// Torrent file.
#[derive(Debug)]
pub struct File {
    /// Length in bytes.
    pub length: usize,
    /// (In multi-file mode) Subdirectory names, where the last element is the file name.
    pub path: Vec<Vec<u8>>,
    /// Optional MD5 sum.
    pub md5sum: Option<Vec<u8>>,
}

impl TorrentInfo {
    /// Create [`TorrentInfo`] from bencoded dictionary.
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

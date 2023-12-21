// https://wiki.theory.org/BitTorrentSpecification

mod download;
mod info;
mod tracker;

use std::sync::Arc;
use tokio::sync::Mutex;

use super::error::Error;
use crate::prelude::*;
use info::TorrentInfo;

pub use tracker::Tracker;

/// Torrent.
#[derive(Debug, Clone)]
pub struct Torrent {
    /// Torrent info.
    pub info: TorrentInfo,
    /// Tracker URL.
    pub announce: String,
    /// Optional announce list.
    pub announce_list: Option<Vec<Vec<String>>>,
    /// Optional creation date.
    pub creation_date: Option<usize>,
    /// Optional comment.
    pub comment: Option<String>,
    /// Optional creator name.
    pub created_by: Option<String>,
    /// Optional encoding type.
    pub encoding: Option<String>,

    /// SHA1 hash of info dictionary.
    info_hash: Vec<u8>,
    /// Download buffer.
    buffer: Arc<Mutex<Vec<u8>>>,
    /// Total amount uploaded.
    _uploaded: usize,
}

impl Torrent {
    /// Create [`Torrent`] from bencoded bytes.
    pub fn from_bcode(contents: &[u8]) -> Result<Self, Error> {
        let dictionary = decode(contents)?.try_as::<Dictionary>()?;
        let info_dictionary = dictionary.try_get_as::<Dictionary>("info")?;
        let info = TorrentInfo::from_bcode(info_dictionary.clone())?;
        let announce = String::from_utf8(dictionary.try_get_as::<ByteString>("announce")?.0)?;
        let announce_list = None;
        let creation_date = None;
        let comment = None;
        let created_by = None;
        let encoding = None;
        let info_hash = sha1_smol::Sha1::from(encode::<Dictionary>(&info_dictionary))
            .digest()
            .bytes()
            .to_vec();
        let buffer = Arc::new(Mutex::new(vec![
            0;
            (info.pieces.len() / 20) * info.piece_length
        ]));

        Ok(Torrent {
            info,
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,

            info_hash,
            buffer,
            _uploaded: 0,
        })
    }

    /// Read a pending [`Torrent`] download process stored on disk.
    pub fn from_disk() -> Result<Self, Error> {
        todo!()
    }

    /// Get `info_hash`.
    pub fn get_hash(&self) -> &[u8] {
        self.info_hash.as_slice()
    }

    /// Get length of bitfield.
    pub fn get_bitfield_length(&self) -> usize {
        const SHA1_HASH_LENGTH: usize = 20;
        const BYTE_LENGTH: usize = 8;

        (self.info.pieces.len() / SHA1_HASH_LENGTH) / BYTE_LENGTH
    }
}

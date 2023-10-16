use super::*;
use crate::prelude::*;

impl Torrent {
    /// Create [`Torrent`] from bencoded bytes.
    pub fn parse(contents: &[u8]) -> Result<Self, Error> {
        let dictionary = decode(contents)?.try_as::<Dictionary>()?;
        let info = dictionary.try_get_as::<Dictionary>("info")?;
        let announce = String::from_utf8(dictionary.try_get_as::<ByteString>("announce")?.0)?;
        let announce_list = None;
        let creation_date = None;
        let comment = None;
        let created_by = None;
        let encoding = None;
        let info_hash = sha1_smol::Sha1::from(encode(&Value::Dictionary(info.clone())))
            .digest()
            .bytes()
            .to_vec();
        let tracker = Tracker::with(&info_hash);

        Ok(Torrent {
            info: TorrentInfo::from_dictionary(info)?,
            announce,
            announce_list,
            creation_date,
            comment,
            created_by,
            encoding,

            info_hash,
            tracker,
        })
    }
}

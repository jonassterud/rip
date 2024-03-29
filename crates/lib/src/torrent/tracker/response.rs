use crate::error::Error;
use crate::prelude::*;

/// Tracker response.
#[derive(Debug)]
pub struct TrackerResponse {
    /// Number of seconds to wait between regular rerequests.
    interval: usize,
    /// List of peers.
    peers: Vec<Peer>,
}

impl TrackerResponse {
    /// Create [`TrackerResponse`] from bytes.
    pub fn from_bytes(contents: &[u8]) -> Result<Self, Error> {
        let dict = decode(contents)?.try_as::<Dictionary>()?;

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

#[test]
fn test_tracker_response_from_bytes() {
    let bytes = b"d8:completei64e10:incompletei1e8:intervali1800e5:peersld2:ip37:2606:6080:1001:12:257a:8b87:f80d:75797:peer id20:-TR4030-0vjbp0s2z68f4:porti61406eed2:ip14:185.125.190.597:peer id20:T03I--00Y-FEdyCcD9xB4:porti6930eeee";
    TrackerResponse::from_bytes(bytes).unwrap();

}

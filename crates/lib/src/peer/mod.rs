use crate::error::Error;
use crate::prelude::*;

/// Torrent peer.
#[derive(Debug)]
pub struct Peer {
    /// Peer ID.
    id: Vec<u8>,
    /// IP address.
    ip: Vec<u8>,
    /// IP port.
    port: Vec<u8>,
}

impl Peer {
    /// Create [`Peer`] from bencoded dictionary.
    pub fn from_dictionary(dictionary: &Dictionary) -> Result<Self, Error> {
        let id = dictionary.try_get_as::<ByteString>("peer id")?.0;
        let ip = dictionary.try_get_as::<ByteString>("ip")?.0;
        let port = dictionary.try_get_as::<ByteString>("port")?.0;

        Ok(Self { id, ip, port })
    }
}

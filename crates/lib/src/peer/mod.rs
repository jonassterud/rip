use crate::error::Error;
use crate::prelude::*;

pub struct Peer {
    id: Vec<u8>,
    ip: Vec<u8>,
    port: Vec<u8>,
}

impl Peer {
    pub fn from_dictionary(dictionary: &Dictionary) -> Result<Self, Error> {
        let id = dictionary.try_get_as::<ByteString>("peer id")?.0;
        let ip = dictionary.try_get_as::<ByteString>("ip")?.0;
        let port = dictionary.try_get_as::<ByteString>("port")?.0;

        Ok(Self { id, ip, port })
    }
}

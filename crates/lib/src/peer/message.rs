use crate::error::Error;

#[derive(Debug)]
#[repr(u8)]
pub enum PeerMessage {
    Choke,
    ///
    Unchoke,
    ///
    Interested,
    ///
    NotInterested,
    /// `index`
    Have(usize),
    /// `bitfield`
    Bitfield(Vec<u8>),
    /// `index`, `begin`, `length`
    Request(usize, usize, usize),
    /// `index`, `begin`, `piece`
    Piece(usize, usize, usize),
    /// `index`, `begin`, `length`
    Cancel(usize, usize, usize),
}

impl PeerMessage {
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn try_from_bytes(contents: &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

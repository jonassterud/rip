use crate::{error::Error, helpers};

#[derive(Debug, PartialEq, Eq)]
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
    Have(u32),
    /// `bitfield`
    Bitfield(Vec<u8>),
    /// `index`, `begin`, `length`
    Request(u32, u32, u32),
    /// `index`, `begin`, `piece`
    Piece(u32, u32, Vec<u8>),
    /// `index`, `begin`, `length`
    Cancel(u32, u32, u32),
}

impl From<&PeerMessage> for u8 {
    fn from(value: &PeerMessage) -> Self {
        match value {
            &PeerMessage::Choke => 0,
            &PeerMessage::Unchoke => 1,
            &PeerMessage::Interested => 2,
            &PeerMessage::NotInterested => 3,
            &PeerMessage::Have(_) => 4,
            &PeerMessage::Bitfield(_) => 5,
            &PeerMessage::Request(_, _, _) => 6,
            &PeerMessage::Piece(_, _, _) => 7,
            &PeerMessage::Cancel(_, _, _) => 8,
        }
    }
}

impl From<u8> for PeerMessage {
    fn from(value: u8) -> Self {
        match value {
            0 => PeerMessage::Choke,
            1 => PeerMessage::Unchoke,
            2 => PeerMessage::Interested,
            3 => PeerMessage::NotInterested,
            4 => PeerMessage::Have(0),
            5 => PeerMessage::Bitfield(Vec::new()),
            6 => PeerMessage::Request(0, 0, 0),
            7 => PeerMessage::Piece(0, 0, Vec::new()),
            8 => PeerMessage::Cancel(0, 0, 0),
            _ => panic!("unexpected peer message value: {value}"),
        }
    }
}

impl PeerMessage {
    /// Get inner message (if any) as bytes.
    fn message_as_bytes(&self) -> Vec<u8> {
        match self {
            PeerMessage::Choke => Vec::new(),
            PeerMessage::Unchoke => Vec::new(),
            PeerMessage::Interested => Vec::new(),
            PeerMessage::NotInterested => Vec::new(),
            PeerMessage::Have(f1) => f1.to_be_bytes().to_vec(),
            PeerMessage::Bitfield(f1) => f1.clone(),
            PeerMessage::Request(f1, f2, f3) => {
                [f1.to_be_bytes(), f2.to_be_bytes(), f3.to_be_bytes()].concat()
            }
            PeerMessage::Piece(f1, f2, f3) => {
                let mut tmp = [f1.to_be_bytes(), f2.to_be_bytes()].concat();
                tmp.append(&mut f3.clone());

                tmp
            }
            PeerMessage::Cancel(f1, f2, f3) => {
                [f1.to_be_bytes(), f2.to_be_bytes(), f3.to_be_bytes()].concat()
            }
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let id = self.into();
        let mut payload = self.message_as_bytes();
        let mut length = (1 + payload.len() as u32).to_be_bytes().to_vec();

        out.append(&mut length);
        out.push(id);
        out.append(&mut payload);

        out
    }

    pub fn try_from_bytes(contents: &[u8]) -> Result<Self, Error> {
        let variant = contents
            .get(4)
            .ok_or_else(|| Error::Peer(format!("missing message variant")))?;
        let p1 = helpers::try_from_slice(&contents, 5, &u32::from_be_bytes);
        let p2 = helpers::try_from_slice(&contents, 9, &u32::from_be_bytes);
        let p3 = helpers::try_from_slice(&contents, 13, &u32::from_be_bytes);

        match (*variant).into() {
            PeerMessage::Choke => Ok(PeerMessage::Choke),
            PeerMessage::Unchoke => Ok(PeerMessage::Unchoke),
            PeerMessage::Interested => Ok(PeerMessage::Interested),
            PeerMessage::NotInterested => Ok(PeerMessage::NotInterested),
            PeerMessage::Have(_) => Ok(PeerMessage::Have(p1?)),
            PeerMessage::Bitfield(_) => Ok(PeerMessage::Bitfield(contents[5..].to_vec())),
            PeerMessage::Request(_, _, _) => Ok(PeerMessage::Request(p1?, p2?, p3?)),
            PeerMessage::Piece(_, _, _) => Ok(PeerMessage::Piece(p1?, p2?, contents[13..].to_vec())),
            PeerMessage::Cancel(_, _, _) => Ok(PeerMessage::Cancel(p1?, p2?, p3?)),
        }
    }
}

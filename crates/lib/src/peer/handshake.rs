use crate::error::Error;

/// Peer handshake.
#[derive(Debug, PartialEq, Eq)]
pub struct PeerHandshake(Vec<u8>);

impl PeerHandshake {
    /// Create a new handshake.
    pub fn new(hash: Vec<u8>, id: [u8; 20]) -> Self {
        let mut bytes = Vec::new();

        bytes.push(19);
        bytes.append(&mut b"BitTorrent protocol".to_vec());
        bytes.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]);
        bytes.append(&mut hash.clone());
        bytes.append(&mut id.to_vec());

        Self(bytes)
    }

    /// Get handshake as bytes.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }

    pub fn verify(&self, bytes: &[u8]) -> Result<(), Error> {
        let hash = bytes.get(bytes.len() - 40..bytes.len() - 20);

        if hash.is_some() && hash == self.0.get(self.0.len() - 40..self.0.len() - 20) {
            Ok(())
        } else {
            Err(Error::Peer(format!("handshake failed")))
        }
    }
}

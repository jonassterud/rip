mod handshake;
mod message;

use crate::error::Error;
use crate::prelude::*;
use futures::future::try_join_all;
pub use handshake::PeerHandshake;
pub use message::PeerMessage;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc as std_mpsc;
use std::sync::Arc;
use tokio::sync::{mpsc as tokio_mpsc, OwnedMutexGuard};
use tokio::sync::{Mutex, MutexGuard};
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct Peer(pub Arc<Mutex<InnerPeer>>);

impl Peer {
    /// Create [`Peer`] from bencoded dictionary.
    pub fn from_bcode(dictionary: &Dictionary) -> Result<Self, Error> {
        let inner = InnerPeer::from_bcode(dictionary)?;
        let outer = Self(Arc::new(Mutex::new(inner)));

        Ok(outer)
    }

    /// Get lock to [`InnerPeer`].
    pub async fn inner<'a>(&'a self) -> MutexGuard<'a, InnerPeer> {
        self.0.lock().await
    }

    pub async fn inner_owned(&self) -> OwnedMutexGuard<InnerPeer> {
        self.0.clone().lock_owned().await
    } 
}

/// Torrent peer.
#[derive(Debug)]
pub struct InnerPeer {
    /// Peer ID.
    id: Vec<u8>,
    /// IP address.
    ip: Vec<u8>,
    /// IP port.
    port: u16,

    am_choking: bool,
    am_interested: bool,
    peer_choking: bool,
    peer_interested: bool,
    bitfield: Vec<u8>,

    /// Outgoing and incoming connection tasks (should be safe to unwrap).
    _tasks: Option<Vec<JoinHandle<Result<(), Error>>>>,
    /// TCP stream.
    _stream: Option<Arc<Mutex<TcpStream>>>,
    /// Outgoing message sender.
    _outgoing: Option<tokio_mpsc::UnboundedSender<PeerMessage>>,
    /// Incoming message receiver.
    _incoming: Option<std_mpsc::Receiver<PeerMessage>>,
}

impl InnerPeer {
    /// Create [`InnerPeer`] from bencoded dictionary.
    pub fn from_bcode(dictionary: &Dictionary) -> Result<Self, Error> {
        let id = dictionary.try_get_as::<ByteString>("peer id")?.0;
        let ip = dictionary.try_get_as::<ByteString>("ip")?.0;
        let port = dictionary.try_get_as::<Integer>("port")?.0 as u16;

        Ok(Self {
            id,
            ip,
            port,

            am_choking: true,
            am_interested: false,
            peer_choking: true,
            peer_interested: false,
            bitfield: Vec::new(),

            _tasks: Some(Vec::new()),
            _stream: None,
            _outgoing: None,
            _incoming: None,
        })
    }

    /// Connect and start listening for incoming and outgoing messages.
    pub fn connect(&mut self, buffer_size: usize, hash: &[u8], id: &[u8]) -> Result<(), Error> {
        // Connect and initialize a stream
        let address = (
            self.ip
                .iter()
                .cloned()
                .map(|b| b as char)
                .collect::<String>(),
            self.port,
        );

        // TODO: Fix IPV6: https://users.rust-lang.org/t/how-to-connect-to-an-ipv6-link-local-address/54372/3
        if address.0.contains(":") {
            return Err(Error::Peer(format!("ipv6 not supported")))
        } else {
            println!("{address:?}")
        }

        self._stream = Some(Arc::new(Mutex::new(TcpStream::connect(address)?)));

        // Create handshake
        let handshake = PeerHandshake::new(hash, id);
        let mut handshake_bytes = handshake.as_bytes();

        // Create an outgoing task
        let outgoing_stream = self._stream.clone().unwrap();
        let (outgoing_s, mut outgoing_r) = tokio_mpsc::unbounded_channel::<PeerMessage>();
        self._outgoing = Some(outgoing_s);

        self._tasks.as_mut().unwrap().push(tokio::spawn(async move {
            outgoing_stream
                .lock()
                .await
                .write_all(&mut handshake_bytes)?;

            while let Some(message) = outgoing_r.recv().await {
                outgoing_stream
                    .lock()
                    .await
                    .write_all(&mut message.as_bytes())?;
            }

            Ok(())
        }));

        // Create an incoming task
        let incoming_stream = self._stream.clone().unwrap();
        let (incoming_s, incoming_r) = std_mpsc::sync_channel::<PeerMessage>(buffer_size);
        self._incoming = Some(incoming_r);
       
        self._tasks.as_mut().unwrap().push(tokio::spawn(async move {
            let mut handshake_buffer = [0_u8; 49 + 19];
            incoming_stream
                .lock()
                .await
                .read_exact(&mut handshake_buffer)?;
            handshake.verify(&handshake_buffer)?;

            println!("handshaked");

            loop {
                let mut incoming_stream = incoming_stream.lock().await;

                let mut length_buffer = [0_u8; 4];
                incoming_stream.read_exact(&mut length_buffer)?;

                let payload_length = u32::from_be_bytes(length_buffer) as usize;
                let mut payload_buffer = vec![0_u8; payload_length];
                incoming_stream.read_exact(&mut payload_buffer)?;

                let mut message_bytes = length_buffer.to_vec();
                message_bytes.append(&mut payload_buffer);

                let message = PeerMessage::try_from_bytes(&message_bytes)?;

                incoming_s.send(message)?;
            }
        }));

        Ok(())
    }

    pub fn handle_messages(&mut self) -> Result<(), Error> {
        let incoming_r = self
            ._incoming
            .take()
            .ok_or_else(|| Error::Peer(format!("message receiver missing")))?;
        
        self._tasks.as_mut().unwrap().push(tokio::spawn(async move {
            while let Ok(message) = incoming_r.recv() {
                match message {
                    PeerMessage::Choke => self.peer_choking = true,
                    PeerMessage::Unchoke => self.peer_choking = false,
                    PeerMessage::Interested => self.peer_interested = true,
                    PeerMessage::NotInterested => self.peer_interested = false,
                    PeerMessage::Have(_) => todo!(),
                    PeerMessage::Bitfield(bitfield) => self.bitfield = bitfield,
                    PeerMessage::Request(_, _, _) => todo!(),
                    PeerMessage::Piece(_, _, _) => todo!(),
                    PeerMessage::Cancel(_, _, _) => todo!(),
                }
            }

            Ok(())
        }));

        Ok(())
    }

    /// Awaits all tasks.
    pub async fn join(&mut self) -> Result<(), Error> {
        try_join_all(self._tasks.take().unwrap_or_default()).await?;

        Ok(())
    }

    /// Send a message to the peer (by adding it to the send queue).
    pub fn send(&mut self, message: PeerMessage) -> Result<(), Error> {
        self._outgoing
            .as_mut()
            .ok_or_else(|| Error::Peer(format!("failed sending message (message:?)")))?
            .send(message)?;

        Ok(())
    }
}

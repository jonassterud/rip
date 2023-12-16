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
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[derive(Debug, Clone)]
pub struct Peer(pub Arc<Mutex<InnerPeer>>);

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

impl Peer {
    /// Create [`Peer`] from bencoded dictionary.
    pub fn from_bcode(dictionary: &Dictionary) -> Result<Self, Error> {
        let id = dictionary.try_get_as::<ByteString>("peer id")?.0;
        let ip = dictionary.try_get_as::<ByteString>("ip")?.0;
        let port = dictionary.try_get_as::<Integer>("port")?.0 as u16;

        Ok(Self(Arc::new(Mutex::new(InnerPeer {
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
        }))))
    }

    /// Connect and start listening for incoming and outgoing messages.
    pub async fn connect(
        &mut self,
        buffer_size: usize,
        hash: &[u8],
        id: &[u8],
    ) -> Result<(), Error> {
        let mut inner = self.0.lock().await;

        // Connect and initialize a stream
        let address = (
            inner
                .ip
                .iter()
                .cloned()
                .map(|b| b as char)
                .collect::<String>(),
            inner.port,
        );

        // TODO: Fix IPV6: https://users.rust-lang.org/t/how-to-connect-to-an-ipv6-link-local-address/54372/3
        if address.0.contains(":") {
            return Err(Error::Peer(format!("ipv6 not supported")));
        } else {
            println!("{address:?}")
        }

        inner._stream = Some(Arc::new(Mutex::new(TcpStream::connect(address)?)));

        // Create handshake
        let handshake = PeerHandshake::new(hash, id);
        let mut handshake_bytes = handshake.as_bytes();

        // Create an outgoing task
        let outgoing_stream = inner._stream.clone().unwrap();
        let (outgoing_s, mut outgoing_r) = tokio_mpsc::unbounded_channel::<PeerMessage>();
        inner._outgoing = Some(outgoing_s);

        inner
            ._tasks
            .as_mut()
            .unwrap()
            .push(tokio::spawn(async move {
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
        let incoming_stream = inner._stream.clone().unwrap();
        let (incoming_s, incoming_r) = std_mpsc::sync_channel::<PeerMessage>(buffer_size);
        inner._incoming = Some(incoming_r);

        inner
            ._tasks
            .as_mut()
            .unwrap()
            .push(tokio::spawn(async move {
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

    pub async fn handle_messages(&mut self) -> Result<(), Error> {
        let mut inner = self.0.lock().await;

        let incoming_r = inner
            ._incoming
            .take()
            .ok_or_else(|| Error::Peer(format!("message receiver missing")))?;

        let self_clone = self.clone();
        inner
            ._tasks
            .as_mut()
            .unwrap()
            .push(tokio::spawn(async move {
                while let Ok(message) = incoming_r.recv() {
                    let mut inner_clone = self_clone.0.lock().await;

                    match message {
                        PeerMessage::Choke => inner_clone.peer_choking = true,
                        PeerMessage::Unchoke => inner_clone.peer_choking = false,
                        PeerMessage::Interested => inner_clone.peer_interested = true,
                        PeerMessage::NotInterested => inner_clone.peer_interested = false,
                        PeerMessage::Have(_) => todo!(),
                        PeerMessage::Bitfield(bitfield) => inner_clone.bitfield = bitfield,
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
        let tasks = self.0.lock().await._tasks.take().unwrap_or_default();
        try_join_all(tasks).await?;

        Ok(())
    }

    /// Send a message to the peer (by adding it to the send queue).
    pub async fn send(&mut self, message: PeerMessage) -> Result<(), Error> {
        let mut inner = self.0.lock().await;

        inner
            ._outgoing
            .as_mut()
            .ok_or_else(|| Error::Peer(format!("failed sending message (message:?)")))?
            .send(message)?;

        Ok(())
    }
}

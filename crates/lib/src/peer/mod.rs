mod handshake;
mod message;

use crate::error::Error;
use crate::prelude::*;
pub use handshake::PeerHandshake;
pub use message::PeerMessage;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::mpsc as std_mpsc;
use std::sync::Arc;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Torrent peer.
#[derive(Debug)]
pub struct Peer {
    /// Peer ID.
    id: Vec<u8>,
    /// IP address.
    ip: Vec<u8>,
    /// IP port.
    port: u16,

    /// TCP stream.
    _stream: Option<Arc<Mutex<TcpStream>>>,
    /// Outgoing and incoming connection tasks.
    _tasks: (
        Option<JoinHandle<Result<(), Error>>>,
        Option<JoinHandle<Result<(), Error>>>,
    ),
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

        Ok(Self {
            id,
            ip,
            port,

            _stream: None,
            _tasks: (None, None),
            _outgoing: None,
            _incoming: None,
        })
    }

    /// Connect and start listening for incoming and outgoing messages.
    pub fn connect(
        &mut self,
        buffer_size: usize,
        hash: &[u8],
        id: &[u8],
    ) -> Result<(), Error> {
        // Connect and initialize a stream
        let address = (
            self.ip
                .iter()
                .cloned()
                .map(|b| b as char)
                .collect::<String>(),
            self.port
        );

        // TODO: Fix IPV6: https://users.rust-lang.org/t/how-to-connect-to-an-ipv6-link-local-address/54372/3
        if address.0.contains(":") {
            return Ok(());
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

        self._tasks.0 = Some(tokio::spawn(async move {
            {
                outgoing_stream
                    .lock()
                    .await
                    .write_all(&mut handshake_bytes)?;

                println!("handshake sent");
            }

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
        self._tasks.1 = Some(tokio::spawn(async move {
            {
                let mut handshake_buffer = [0_u8; 49 + 19];
                incoming_stream
                    .lock()
                    .await
                    .read_exact(&mut handshake_buffer)?;
                handshake.verify(&handshake_buffer)?;

                println!("handshake verified");
            }

            loop {
                let mut incoming_stream = incoming_stream.lock().await; // todo: will this block?

                let mut length_buffer = [0_u8; 4];
                incoming_stream.read_exact(&mut length_buffer)?;

                let message_length = u32::from_be_bytes(length_buffer) as usize;
                let mut message_buffer = vec![0_u8; message_length];
                incoming_stream.read_exact(&mut message_buffer)?;

                let message = PeerMessage::try_from_bytes(&message_buffer)?;
                println!("{message:?}");

                incoming_s.send(message)?;
            }
        }));

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

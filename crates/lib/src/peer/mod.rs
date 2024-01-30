mod handshake;
mod message;

use crate::error::Error;
use crate::prelude::*;
use futures::future::try_join_all;
pub use handshake::PeerHandshake;
pub use message::PeerMessage;
use std::io::{Read, Write};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use std::net::{TcpStream, SocketAddr, IpAddr};
use socket2::{Socket, Domain, Type, Protocol};

#[derive(Debug, Clone)]
pub struct Peer(pub Arc<Mutex<InnerPeer>>);

/// Torrent peer.
#[derive(Debug)]
pub struct InnerPeer {
    /// Peer ID.
    id: Vec<u8>,
    /// IP address.
    pub ip: Vec<u8>,
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
    _stream: Option<Arc<Mutex<Socket>>>,
    /// Outgoing message sender.
    _outgoing: Option<mpsc::UnboundedSender<PeerMessage>>,
    /// Incoming message receiver.
    _incoming: Option<mpsc::Receiver<PeerMessage>>,
}

impl Peer {
    /// Create [`Peer`] from bencoded dictionary.
    pub fn from_bcode(dictionary: &Dictionary, bitfield_length: usize) -> Result<Self, Error> {
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
            bitfield: vec![0_u8; bitfield_length],

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
        let ip = inner
            .ip
            .iter()
            .cloned()
            .map(|b| b as char)
            .collect::<String>();

        let domain = if ip.contains(':') {
            Domain::IPV6
        } else {
            Domain::IPV4
        };

        println!("Before: {:?}", SocketAddr::new(IpAddr::from_str(&ip).unwrap(), inner.port));

        let socket = Socket::new(domain, Type::STREAM, Some(Protocol::TCP));

        //println!("{socket:?}");
        let socket = socket?;
        let address: SocketAddr = SocketAddr::new(IpAddr::from_str(&ip).unwrap(), inner.port);
        let t = socket.connect(&address.into());

        //println!("Trying: {address:?}");
        println!("{t:?}");

        inner._stream = Some(Arc::new(Mutex::new(socket)));

        println!("Connecting to: {:?}::{:?}", ip, inner.port);

        // Create handshake
        let handshake = PeerHandshake::new(hash, id);
        let handshake_bytes = handshake.as_bytes();

        // Create an outgoing task
        let outgoing_stream = inner._stream.clone().unwrap();
        let (outgoing_s, mut outgoing_r) = mpsc::unbounded_channel::<PeerMessage>();
        inner._outgoing = Some(outgoing_s.clone());

        inner
            ._tasks
            .as_mut()
            .unwrap()
            .push(tokio::spawn(async move {
                outgoing_stream.lock().await.write_all(&handshake_bytes)?;

                while let Some(message) = outgoing_r.recv().await {
                    println!("Sending: {message:?}");

                    outgoing_stream
                        .lock()
                        .await
                        .send(&message.as_bytes())?;
                }

                Ok(())
            }));

        // TODO: temp start
        println!("sending interested and unchoke...");
        outgoing_s.send(PeerMessage::Interested)?;
        outgoing_s.send(PeerMessage::Unchoke)?;
        // TODO: temp end

        // Create an incoming task
        let incoming_stream = inner._stream.clone().unwrap();
        let (incoming_s, incoming_r) = mpsc::channel::<PeerMessage>(buffer_size);
        inner._incoming = Some(incoming_r);

        inner
            ._tasks
            .as_mut()
            .unwrap()
            .push(tokio::spawn(async move {
                {
                    let mut handshake_buffer = [0_u8; 49 + 19];
                    incoming_stream
                        .lock()
                        .await
                        .read_exact(&mut handshake_buffer)?;
                    handshake.verify(&handshake_buffer)?;
                }

                println!("handshaked!");

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

                    incoming_s.send(message).await?;
                }
            }));

        Ok(())
    }

    pub async fn handle_messages(
        &mut self,
        sender: mpsc::UnboundedSender<(u32, u32, Vec<u8>)>,
    ) -> Result<(), Error> {
        let mut inner = self.0.lock().await;

        let mut incoming_r = inner
            ._incoming
            .take()
            .ok_or_else(|| Error::Peer(format!("message receiver missing")))?;

        while let Some(message) = incoming_r.recv().await {
            let mut inner_clone = self.0.lock().await;

            match message {
                PeerMessage::Choke => inner_clone.peer_choking = true,
                PeerMessage::Unchoke => inner_clone.peer_choking = false,
                PeerMessage::Interested => inner_clone.peer_interested = true,
                PeerMessage::NotInterested => inner_clone.peer_interested = false,
                PeerMessage::Have(index) => {
                    let (byte_index, bit_index) = (index as usize / 8, index % 8);
                    inner_clone.bitfield[byte_index] |= 128_u8 >> bit_index;
                }
                PeerMessage::Bitfield(bitfield) => inner_clone.bitfield = bitfield,
                PeerMessage::Request(_, _, _) => {
                    // todo
                    println!("peer requested!")
                }
                PeerMessage::Piece(index, begin, piece) => {
                    sender.send((index, begin, piece))?;
                }
                PeerMessage::Cancel(_, _, _) => {
                    // todo
                    println!("peer cancelled!")
                }
            }
        }

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

/*
  Copyright (C) 2018-2020 The Purple Core Developers.
  This file is part of the Purple Core Library.

  The Purple Core Library is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The Purple Core Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the Purple Core Library. If not, see <http://www.gnu.org/licenses/>.
*/

use crate::bootstrap::cache::BootstrapCache;
use crate::client_request::ClientRequest;
use crate::error::NetworkErr;
use crate::packet::Packet;
use crate::priority::NetworkPriority;
use crate::util::FuturesIoSock;
use crate::validation::validator::ProtocolValidator;
use crypto::{gen_kx_keypair, KxPublicKey as Pk, KxSecretKey as Sk, SessionKey};
use crypto::{Hash, NodeId};
use flume::Sender;
use std::default::Default;
use std::fmt;
use std::hash::{Hash as HashTrait, Hasher};
use std::net::SocketAddr;
use std::sync::atomic::AtomicU64;
use triomphe::Arc;
use yamux::Control;

#[cfg(test)]
use parking_lot::Mutex;

#[cfg(test)]
use timer::{Guard, Timer};

#[derive(Clone, Debug, Copy)]
pub enum ConnectionType {
    Client,
    Server,
}

/// Size of the outbound buffer.
pub const OUTBOUND_BUF_SIZE: usize = 10000;

#[derive(Clone)]
pub struct Peer {
    /// The id of the peer
    ///
    /// An option is used in order to store the peer's
    /// session before a connect packet containing the
    /// node's id has been received.
    pub id: Option<NodeId>,

    /// The type of the connection. Can be
    /// either `Client` or `Server`.
    ///
    /// A connection type is `Client` when
    /// we are the one connecting.
    ///
    /// A connection type is `Server` when
    /// a peer connects to us.
    pub connection_type: ConnectionType,

    /// The ip address of the peer
    pub ip: SocketAddr,

    /// Time in milliseconds since the peer has last sent a message.
    pub last_seen: Arc<AtomicU64>,

    /// Time in milliseconds since we have sent a ping to the peer.
    pub last_ping: Arc<AtomicU64>,

    /// Bytes read in the current second
    pub bytes_read: Arc<AtomicU64>,

    /// Bytes read in the preceding second
    pub past_bytes_read: Arc<AtomicU64>,

    /// Bytes write in the current second
    pub bytes_write: Arc<AtomicU64>,

    /// Bytes wrote in the preceding second
    pub past_bytes_write: Arc<AtomicU64>,

    /// Whether the peer has sent a `Connect` packet or not.
    pub sent_connect: bool,

    /// Session generated public key
    pub pk: Pk,

    /// Session generated secret key
    pub(crate) sk: Sk,

    /// Our encryption key
    pub(crate) rx: Option<SessionKey>,

    /// The peer's encryption key
    pub(crate) tx: Option<SessionKey>,

    /// Associated protocol validator
    pub(crate) validator: ProtocolValidator,

    /// Buffer storing packets that are to be
    /// sent to the peer with low priority.
    pub low_outbound_buffer: Option<Sender<(Vec<u8>, ClientRequest)>>,

    /// Buffer storing packets that are to be
    /// sent to the peer with medium priority.
    pub medium_outbound_buffer: Option<Sender<(Vec<u8>, ClientRequest)>>,

    /// Buffer storing packets that are to be
    /// sent to the peer with high priority.
    pub high_outbound_buffer: Option<Sender<(Vec<u8>, ClientRequest)>>,

    #[cfg(not(test))]
    /// Control for the tcp connection
    pub tcp_control: Option<Control>,

    #[cfg(test)]
    pub(crate) timeout_guard: Option<Guard>,

    #[cfg(test)]
    pub(crate) timer: Option<Arc<Mutex<Timer>>>,

    #[cfg(test)]
    pub(crate) send_ping: bool,
}

impl fmt::Debug for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Peer(id: {:?}, ip: {:?})", self.id, self.ip)
    }
}

impl Peer {
    pub fn new(
        id: Option<NodeId>,
        ip: SocketAddr,
        connection_type: ConnectionType,
        low_outbound_buffer: Option<Sender<(Vec<u8>, ClientRequest)>>,
        medium_outbound_buffer: Option<Sender<(Vec<u8>, ClientRequest)>>,
        high_outbound_buffer: Option<Sender<(Vec<u8>, ClientRequest)>>,
        bootstrap_cache: BootstrapCache,
    ) -> Peer {
        let (pk, sk) = gen_kx_keypair();

        Peer {
            id,
            ip,
            pk,
            sk,
            rx: None,
            tx: None,
            sent_connect: false,
            connection_type,
            low_outbound_buffer,
            medium_outbound_buffer,
            high_outbound_buffer,
            last_seen: Arc::new(AtomicU64::new(0)),
            last_ping: Arc::new(AtomicU64::new(0)),
            bytes_read: Arc::new(AtomicU64::new(0)),
            past_bytes_read: Arc::new(AtomicU64::new(0)),
            bytes_write: Arc::new(AtomicU64::new(0)),
            past_bytes_write: Arc::new(AtomicU64::new(0)),
            validator: ProtocolValidator::new(bootstrap_cache),

            #[cfg(not(test))]
            tcp_control: None,

            #[cfg(test)]
            timeout_guard: None,

            #[cfg(test)]
            timer: None,

            #[cfg(test)]
            send_ping: true,
        }
    }

    /// Sets the id of the peer to the given value
    pub fn set_id(&mut self, id: NodeId) {
        self.id = Some(id);
    }

    /// Sets the session keys associated with the peer
    pub fn set_session_keys(&mut self, rx: SessionKey, tx: SessionKey) {
        self.rx = Some(rx);
        self.tx = Some(tx);
    }

    /// Attempts to place a packet in the outbound buffer of a `Peer`.
    pub fn send_packet<P: Packet>(
        &self,
        packet: &P,
        priority: NetworkPriority,
    ) -> Result<(), NetworkErr> {
        let mut sender = match priority {
            NetworkPriority::Low => self.low_outbound_buffer.as_ref().unwrap().clone(),
            NetworkPriority::Medium => self.medium_outbound_buffer.as_ref().unwrap().clone(),
            NetworkPriority::High => self.high_outbound_buffer.as_ref().unwrap().clone(),
        };

        let req = packet.to_client_request().unwrap();
        let packet = packet.to_bytes();

        sender.try_send((packet, req)).map_err(|err| {
            debug!("Packet sending error: {:?}", err);
            NetworkErr::CouldNotSend
        })
    }
}

impl PartialEq for Peer {
    fn eq(&self, other: &Peer) -> bool {
        match (&self.id, &other.id) {
            // Check both ids and ips
            (Some(id1), Some(id2)) => id1 == id2 && self.ip == other.ip,
            // Fallback to just comparing ips
            (_, _) => self.ip == other.ip,
        }
    }
}

impl Eq for Peer {}

impl HashTrait for Peer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(id) = &self.id {
            id.hash(state);
        }
        self.ip.hash(state);
    }
}

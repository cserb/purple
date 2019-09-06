/*
  Copyright (C) 2018-2019 The Purple Core Developers.
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

use crate::chain::*;
use crate::error::NetworkErr;
use crate::packet::Packet;
use crate::peer::Peer;
use crypto::NodeId;
use std::net::SocketAddr;
use parking_lot::RwLock;
use hashbrown::HashMap;
use std::sync::Arc;

#[cfg(not(test))]
use futures::sync::mpsc::Sender;

#[cfg(test)]
use std::sync::mpsc::Sender;

/// Generic network layer interface.
pub trait NetworkInterface {
    /// Attempts to connect to the peer with the given ip.
    fn connect(&mut self, address: &SocketAddr) -> Result<(), NetworkErr>;

    /// Attempts to connect to a previously encountered peer
    fn connect_to_known(&self, peer: &NodeId) -> Result<(), NetworkErr>;

    /// Returns true if the network has the given address in its peer list.
    fn is_connected_to(&self, address: &SocketAddr) -> bool;

    /// Disconnects from the peer with the given `NodeId`.
    fn disconnect(&mut self, peer: &NodeId) -> Result<(), NetworkErr>;

    /// Disconnects from the peer with the given ip address.
    fn disconnect_from_ip(&mut self, ip: &SocketAddr) -> Result<(), NetworkErr>;

    /// Sends a packet to a specific peer.
    fn send_to_peer(&self, peer: &SocketAddr, packet: Vec<u8>) -> Result<(), NetworkErr>;

    /// Sends a packet to all peers.
    fn send_to_all(&self, packet: &[u8]) -> Result<(), NetworkErr>;

    /// Sends a packet to all peers except the given address.
    fn send_to_all_except(&self, exception: &SocketAddr, packet: &[u8]) -> Result<(), NetworkErr>;

    /// Signs a packet and sends it to all peers.
    fn send_to_all_unsigned<P: Packet>(&self, packet: &mut P) -> Result<(), NetworkErr>;

    /// Signs a packet and sends it to all peers
    /// except the peer with the given address.
    fn send_to_all_unsigned_except<P: Packet>(
        &self,
        exception: &SocketAddr,
        packet: &mut P,
    ) -> Result<(), NetworkErr>;

    /// Attempts to send a packet to the specific peer. This
    /// function will also sign the packet if it does not yet
    /// have a signature and it will also serialize it to binary.
    fn send_unsigned<P: Packet>(&self, peer: &SocketAddr, packet: &mut P)
        -> Result<(), NetworkErr>;

    /// Sends a raw packet to a specific peer. This
    /// means that the packet will be un-encrypted.
    fn send_raw(&self, peer: &SocketAddr, packet: Vec<u8>) -> Result<(), NetworkErr>;

    /// This behaves similarly to `send_unsigned()` but it sends a raw packet.
    fn send_raw_unsigned<P: Packet>(
        &self,
        peer: &SocketAddr,
        packet: &mut P,
    ) -> Result<(), NetworkErr>;

    /// Callback that processes each packet that is received from any peer.
    fn process_packet(&mut self, peer: &SocketAddr, packet: &[u8]) -> Result<(), NetworkErr>;

    /// Bans the peer with the node id
    fn ban_peer(&self, peer: &NodeId) -> Result<(), NetworkErr>;

    /// Bans any further connections from the given ip.
    fn ban_ip(&self, peer: &SocketAddr) -> Result<(), NetworkErr>;

    /// Returns a reference to our node id.
    fn our_node_id(&self) -> &NodeId;

    /// Returns a reference to the peer table RwLock.
    fn peers(&self) -> Arc<RwLock<HashMap<SocketAddr, Peer>>>;

    /// Returns a reference to the `HardChain`.
    fn hard_chain_ref(&self) -> HardChainRef;

    /// Returns a reference to the `StateChain`.
    fn state_chain_ref(&self) -> StateChainRef;

    /// Returns a reference to a `HardChain` mpsc sender.
    /// Use this to buffer blocks that are to be appended
    /// to the chain.
    fn hard_chain_sender(&self) -> &Sender<(SocketAddr, Arc<HardBlock>)>;

    /// Returns a reference to a `StateChain` mpsc sender.
    /// Use this to buffer blocks that are to be appended
    /// to the chain.
    fn state_chain_sender(&self) -> &Sender<(SocketAddr, Arc<StateBlock>)>;
}

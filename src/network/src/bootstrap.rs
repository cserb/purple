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

use connection::connect_to_peer;
use futures::stream;
use futures::Future;
use futures::Stream;
use hashdb::HashDB;
use network::Network;
use parking_lot::Mutex;
use persistence::PersistentDb;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::executor::Spawn;

const BOOTNODES: &'static [&'static str] = &["95.179.130.222:44034"];

pub fn bootstrap(
    network: Arc<Mutex<Network>>,
    accept_connections: Arc<AtomicBool>,
    db: PersistentDb,
    max_peers: usize,
) -> Spawn {
    info!("Bootstrapping...");

    let bootstrap_cache_key = crypto::hash_slice(b"bootstrap_cache");

    // Try to first connect to the nodes in the bootstrap cache.
    if let Some(cache) = db.get(&bootstrap_cache_key) {
        let cache: Vec<String> = rlp::decode_list(&cache);
        let cache: Vec<SocketAddr> = cache.iter().map(|addr| addr.parse().unwrap()).collect();

        let peers_to_connect = if cache.len() > max_peers {
            cache[..max_peers].to_vec()
        } else {
            cache
        };

        let network = network.clone();
        let network_clone = network.clone();
        let accept_connections = accept_connections.clone();
        let accept_connections_clone = accept_connections.clone();

        let fut = stream::iter_ok(peers_to_connect)
            .for_each(move |addr| {
                connect_to_peer(network.clone(), accept_connections.clone(), &addr)
            })
            .and_then(move |_| {
                // Connect to bootstrap nodes if we haven't
                // yet reached the maximum amount of peers.
                if network_clone.lock().peer_count() < max_peers {
                    let bootnodes: Vec<SocketAddr> =
                        BOOTNODES.iter().map(|addr| addr.parse().unwrap()).collect();

                    let accept_connections = accept_connections_clone.clone();
                    let network = network_clone.clone();

                    let fut = stream::iter_ok(bootnodes).for_each(move |addr| {
                        connect_to_peer(network.clone(), accept_connections.clone(), &addr)
                    });

                    tokio::spawn(fut);
                    info!("Finished bootstrap");
                    Ok(())
                } else {
                    info!("Finished bootstrap");
                    Ok(())
                }
            });

        tokio::spawn(fut)
    } else {
        let bootnodes: Vec<SocketAddr> =
            BOOTNODES.iter().map(|addr| addr.parse().unwrap()).collect();

        let mut peers_to_connect: Vec<SocketAddr> = Vec::with_capacity(bootnodes.len());

        for addr in bootnodes.iter().take(max_peers) {
            peers_to_connect.push(*addr);
        }

        let accept_connections = accept_connections.clone();
        let network = network.clone();

        let fut = stream::iter_ok(peers_to_connect).for_each(move |addr| {
            connect_to_peer(network.clone(), accept_connections.clone(), &addr)
        });

        tokio::spawn(fut)
    }
}

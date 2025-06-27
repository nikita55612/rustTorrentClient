pub mod conn;

use rand::Rng;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    ops::Deref,
};
use tokio::net::TcpStream;

use crate::error::Result;
use crate::hash::Hash;

const PEER_ID_PREFIX: &'static str = "-UT2210-";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerId(Hash);

impl Deref for PeerId {
    type Target = Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PeerId {
    pub fn new(s: [u8; 20]) -> Self {
        Self(Hash::new(s))
    }

    pub fn gen_new() -> Self {
        Self::gen_with_prefix(PEER_ID_PREFIX.as_bytes().try_into().unwrap())
    }

    pub fn gen_with_prefix(p: &[u8; 8]) -> Self {
        let mut hash = [0u8; 20];
        hash[..8].copy_from_slice(p);

        let rng = rand::rng();
        let iterator = rng
            .sample_iter(rand::distr::Alphanumeric)
            .take(12)
            .enumerate();

        for (i, byte) in iterator {
            hash[8 + i] = byte;
        }

        Self(Hash::new(hash))
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Peer {
    pub socket_addr: SocketAddrV4,
    pub id: Option<PeerId>,
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.socket_addr == other.socket_addr
    }
}

impl Peer {
    pub fn from_bytes(bytes: &[u8; 6]) -> Self {
        let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
        let port = ((bytes[4] as u16) << 8) | (bytes[5] as u16);
        let socket_addr = SocketAddrV4::new(ip, port);

        Peer {
            socket_addr: socket_addr,
            id: None,
        }
    }

    pub async fn connect(&self) -> Result<TcpStream> {
        Ok(TcpStream::connect(self.socket_addr).await?)
    }
}

#[derive(Debug)]
pub struct Peers(Vec<Peer>);

impl Deref for Peers {
    type Target = Vec<Peer>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Peers {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let peers = bytes
            .chunks(6)
            .filter_map(|v| v.try_into().ok().map(Peer::from_bytes))
            .collect();

        Peers(peers)
    }

    pub fn update_peers(&self, bytes: &[u8]) -> () {
        let peers = Self::from_bytes(bytes);

        for peer in peers.iter() {
            if self.get_peer_from_socket_addr(&peer.socket_addr).is_none() {}
        }
    }

    pub fn get_peer_from_id(&self, id: &PeerId) -> Option<&Peer> {
        self.iter().find(|p| p.id.as_ref() == Some(id))
    }

    pub fn get_peer_from_socket_addr(&self, addr: &SocketAddrV4) -> Option<&Peer> {
        self.iter().find(|p| &p.socket_addr == addr)
    }
}

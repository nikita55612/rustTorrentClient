pub mod conn;

use rand::Rng;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    ops::Deref,
};
use tokio::sync::mpsc::Receiver;

use crate::{
    error::Result,
    peer::conn::Conn,
    proto::{handshake::Handshake, message::Message},
};
use crate::{hash::Hash, proto::bitfield::BitField};

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

// 🔄 Полная последовательность
// TCP connect

// Отправка handshake

// Получение handshake

// Получение bitfield

// Отправка interested

// Получение unchoke

// Отправка request

// Получение piece

#[derive(Debug)]
pub struct Peer {
    pub addr: SocketAddrV4,
    pub conn: Option<Conn>,
    pub conn_attempts: usize,
    pub receiver: Option<Receiver<Message>>,
    pub handshake: Option<Handshake>,
    pub bitfield: Option<BitField>,
    pub unchoke: bool,
}

impl PartialEq for Peer {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl Eq for Peer {}

impl Peer {
    pub fn new(addr: SocketAddrV4) -> Self {
        Self {
            addr: addr,
            conn: None,
            conn_attempts: 0,
            receiver: None,
            handshake: None,
            bitfield: None,
            unchoke: false,
        }
    }

    pub fn from_bytes(bytes: &[u8; 6]) -> Self {
        let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
        let port = ((bytes[4] as u16) << 8) | (bytes[5] as u16);
        let addr = SocketAddrV4::new(ip, port);

        Self::new(addr)
    }

    pub async fn try_connect(&mut self) -> Result<()> {
        match Conn::connect(&self.addr).await {
            Ok((conn, receiver)) => {
                self.conn = Some(conn);
                self.receiver = Some(receiver);
                Ok(())
            }
            Err(e) => {
                self.conn_attempts += 1;
                Err(e)
            }
        }
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
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let peers = bytes
            .chunks(6)
            .filter_map(|v| v.try_into().ok().map(Peer::from_bytes))
            .collect();

        Peers(peers)
    }

    pub fn update_from_bytes(&self, bytes: &[u8]) -> () {
        let peers = Self::from_bytes(bytes);

        for peer in peers.iter() {
            if self.get_peer_from_addr(&peer.addr).is_none() {}
        }
    }

    pub fn get_peer_from_addr(&self, addr: &SocketAddrV4) -> Option<&Peer> {
        self.iter().find(|p| &p.addr == addr)
    }
}

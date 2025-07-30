/// https://wiki.theory.org/BitTorrentSpecification#Tracker_Response
/// https://bittorrent.org/beps/bep_0015.html#announce
use crate::error::{Error, Result};
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PeerDict {
    #[serde(default, rename = "peer id")]
    pub peer_id: Option<Vec<u8>>,

    pub ip: String,

    pub port: u16,
}

#[derive(Debug, Clone)]
pub enum Peers {
    BinaryModel(Vec<u8>),
    DictModel(Vec<PeerDict>),
}

impl Peers {
    pub fn len(&self) -> usize {
        match self {
            Self::BinaryModel(v) => v.len(),
            Self::DictModel(d) => d.len(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpAnnounceResponse {
    #[serde(default, rename = "failure reason")]
    failure_reason: Option<String>,

    #[serde(default, rename = "warning message")]
    pub warning_message: Option<String>,

    #[serde(default)]
    pub interval: Option<u64>,

    #[serde(default, rename = "min interval")]
    pub min_interval: Option<u64>,

    #[serde(default, rename = "tracker id")]
    pub tracker_id: Option<String>,

    #[serde(default)]
    pub complete: Option<u32>,

    #[serde(default)]
    pub incomplete: Option<u32>,

    // peers_value -> peers
    #[serde(default, rename = "peers")]
    peers_value: Option<BencodeValue>,

    #[serde(skip)]
    peers: Option<Peers>,
}

impl HttpAnnounceResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut announce_response = serde_bencode::de::from_bytes::<Self>(bytes)?;

        let peers_value = announce_response.peers_value.take();
        if let Some(peers_value) = peers_value {
            match peers_value {
                BencodeValue::Bytes(bytes) => {
                    announce_response.peers.replace(Peers::BinaryModel(bytes));
                }
                BencodeValue::List(list) => {
                    let bytes = serde_bencode::ser::to_bytes(&list)?;
                    announce_response
                        .peers
                        .replace(Peers::DictModel(serde_bencode::de::from_bytes(&bytes)?));
                }
                _ => (),
            }
        }

        Ok(announce_response)
    }

    pub fn check_failure_reason(self) -> Result<Self> {
        match self.failure_reason {
            Some(e) => Result::Err(Error::TrackerFailureReason(e)),
            None => Result::Ok(self),
        }
    }

    pub fn take_peers(&mut self) -> Option<Peers> {
        self.peers.take()
    }
}

#[derive(Debug)]
pub struct Bep15AnnounceResponse {
    pub transaction_id: u32,
    pub interval: u32,
    pub leechers: u32,
    pub seeders: u32,
    pub peers: Vec<SocketAddr>,
}

impl Bep15AnnounceResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 20 {
            return Result::Err(Error::InvalidBep15Response(
                "Announce response too short".into(),
            ));
        }

        let action = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        if action != 1 {
            return Result::Err(Error::InvalidBep15Response(
                "Invalid action in announce response".into(),
            ));
        }

        let transaction_id = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let interval = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let leechers = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        let seeders = u32::from_be_bytes(bytes[16..20].try_into().unwrap());

        let mut peers = Vec::new();
        let mut offset = 20;

        while offset + 6 <= bytes.len() {
            let ip = IpAddr::V4(Ipv4Addr::new(
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ));
            let port = u16::from_be_bytes(bytes[offset + 4..offset + 6].try_into().unwrap());
            peers.push(SocketAddr::new(ip, port));
            offset += 6;
        }

        Ok(Self {
            transaction_id,
            interval,
            leechers,
            seeders,
            peers,
        })
    }
}

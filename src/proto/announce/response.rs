/// https://wiki.theory.org/BitTorrentSpecification#Tracker_Response
/// https://bittorrent.org/beps/bep_0015.html#announce
use crate::error::{Error, Result};
use serde::Deserialize;

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

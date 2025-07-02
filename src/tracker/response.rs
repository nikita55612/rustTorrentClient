/// https://wiki.theory.org/BitTorrentSpecification#Tracker_Response
use crate::error::Result;

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

impl Default for Peers {
    fn default() -> Self {
        Self::BinaryModel(Vec::default())
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackerResponse {
    #[serde(default, rename = "failure reason")]
    pub failure_reason: Option<String>,

    #[serde(default, rename = "warning message")]
    pub warning_message: Option<String>,

    #[serde(default)]
    pub interval: Option<u64>,

    #[serde(default, rename = "min interval")]
    pub min_interval: Option<u64>,

    #[serde(default, rename = "tracker id")]
    pub tracker_id: Option<String>,

    #[serde(default)]
    pub complete: Option<u64>,

    #[serde(default)]
    pub incomplete: Option<u64>,

    #[serde(default, rename = "peers")]
    peers_value: Option<BencodeValue>,

    #[serde(skip)]
    pub peers: Peers,
}

impl TrackerResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut tracker_response = serde_bencode::de::from_bytes::<Self>(bytes)?;

        let peers_value = tracker_response.peers_value.take();
        if let Some(peers_value) = peers_value {
            match peers_value {
                BencodeValue::Bytes(bytes) => {
                    tracker_response.peers = Peers::BinaryModel(bytes);
                }
                BencodeValue::List(list) => {
                    let bytes = serde_bencode::ser::to_bytes(&list)?;
                    tracker_response.peers =
                        Peers::DictModel(serde_bencode::de::from_bytes(&bytes)?);
                }
                _ => (),
            }
        }

        Ok(tracker_response)
    }
}

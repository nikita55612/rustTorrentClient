use serde::{Deserialize, Serialize};
use crate::error::Error;


#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

impl Torrent {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(serde_bencode::de::from_bytes::<Self>(bytes)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: u64,
    pub pieces: serde_bytes::ByteBuf,
    pub length: Option<u64>,
    pub files: Option<Vec<TorrentFile>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentFile {
    pub length: u64,
    pub path: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerResponse {
    pub interval: u32,
    pub peers: serde_bytes::ByteBuf
}

impl TrackerResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(serde_bencode::de::from_bytes::<Self>(bytes)?)
    }
}
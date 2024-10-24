use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};


pub type BencodeValue = serde_bencode::value::Value;


#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

impl Torrent {
    pub fn from_file(bytes: &[u8]) -> Option<Self> {
        serde_bencode::de::from_bytes::<Self>(bytes).ok()
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
    #[serde(default)]
    pub interval: u32,
    #[serde(default)]
    pub peers: serde_bytes::ByteBuf
}

pub struct InfoHash(Vec<u8>);


impl InfoHash {
    pub fn from_file(bytes: &[u8]) -> Option<Self> {
        let decoded: BencodeValue = serde_bencode::de::from_bytes(bytes).ok()?;
        if let BencodeValue::Dict(root_dict) = decoded {
            let info = root_dict.get("info".as_bytes())?;
            let encoded = serde_bencode::ser::to_bytes(info).ok()?;
            let mut hasher = Sha1::new();
            hasher.update(&encoded);
            return Some(InfoHash(hasher.finalize().to_vec()));
        }
        None
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub fn gen_peer_id() -> String {
    "-UT2210-".to_owned() + &Alphanumeric.sample_string(&mut rand::thread_rng(), 12)
}

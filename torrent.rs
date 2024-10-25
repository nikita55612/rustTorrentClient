use std::net::{Ipv4Addr, SocketAddrV4};

use rand::distributions::{Alphanumeric, DistString};
use reqwest::Client;
use sha1::{Digest, Sha1};
use tokio::io::AsyncReadExt;

use crate::bencode;
use crate::error::Error;


const PORT: u16 = 6881;
pub type BencodeValue = serde_bencode::value::Value;


async fn read_file(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

#[derive(Debug)]
pub struct TorrentClient {
    pub torrent: Torrent,
    pub peer_id: String,
    pub handshake: Handshake,
}

impl TorrentClient {
    pub async fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let peer_id = gen_peer_id();
        let torrent = Torrent::from_bytes(bytes, &peer_id).await?;
        let handshake = Handshake::new(
            torrent.info_hash.as_bytes().try_into().unwrap(), 
            peer_id.as_bytes().try_into().unwrap()
        );
        Ok(
            Self {
                torrent,
                peer_id,
                handshake
            }
        )
    }

    pub async fn from_path(path: &str) -> Result<Self, Error> {
        let bytes = read_file(path).await?;
        Self::from_bytes(&bytes).await
    }
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: bencode::Info,
    pub info_hash: InfoHash,
    pub peers_addr: Vec<SocketAddrV4>
}

impl Torrent {
    pub async fn from_bytes(bytes: &[u8], peer_id: &str) -> Result<Self, Error> {
        let de = bencode::Torrent::from_bytes(bytes)?;
        let info_hash = InfoHash::from_bytes(bytes)?;
        let tracker_response = send_tracker_request(&de, &info_hash, peer_id).await?;
        let mut peers_addr = Vec::new();
        for chunk in tracker_response.peers.chunks(6) {
            let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
            let port = ((chunk[4] as u16) << 8) | (chunk[5] as u16);
            let socket_addr = SocketAddrV4::new(ip, port);
            peers_addr.push(socket_addr);
        }
        Ok(
            Self {
                announce: de.announce,
                info: de.info,
                info_hash,
                peers_addr
            }
        )
    }

    pub async fn from_path(path: &str, peer_id: &str) -> Result<Self, Error> {
        let bytes = read_file(path).await?;
        Self::from_bytes(&bytes, peer_id).await
    }
}

async fn send_tracker_request(torrent: &bencode::Torrent, info_hash: &InfoHash, peer_id: &str) -> Result<bencode::TrackerResponse, Error> {
    let client = Client::new();
    let byte_encode = |bytes: &[u8]| -> String {
        bytes.iter()
            .map(|&byte| format!("%{:02X}", byte))
            .collect()
    };
    let url = format!(
        "{}?info_hash={}&peer_id={}&port={}&uploaded=0&downloaded=0&left={}&compact=1&event=started",
        torrent.announce,
        byte_encode(info_hash.as_bytes()),
        urlencoding::encode(peer_id),
        PORT,
        torrent.info.length.unwrap_or(0)
    );
    let response = client.get(&url)
        .header("User-Agent", "uTorrent/2210(25110)")
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();
    Ok(bencode::TrackerResponse::from_bytes(&response)?)
}

#[derive(Debug)]
pub struct InfoHash(Vec<u8>);


impl InfoHash {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let decoded: BencodeValue = serde_bencode::de::from_bytes(bytes)?;
        if let BencodeValue::Dict(root_dict) = decoded {
            let info = root_dict.get("info".as_bytes())
                .ok_or(Error::ParseInfoHashError)?;
            let encoded = serde_bencode::ser::to_bytes(info)?;
            let mut hasher = Sha1::new();
            hasher.update(&encoded);
            return Ok(InfoHash(hasher.finalize().to_vec()));
        }
        Err(Error::ParseInfoHashError)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub fn gen_peer_id() -> String {
    "-UT2210-".to_owned() + &Alphanumeric.sample_string(&mut rand::thread_rng(), 12)
}

#[derive(Debug, Clone)]
pub struct Handshake {
    pub pstr: String,
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub buffer: Vec<u8>,
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        let pstr = "BitTorrent protocol".to_string();
        Self { 
            buffer: handshake_buffer(&pstr, &info_hash, &peer_id),
            pstr,
            info_hash,
            peer_id,
        }
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<Self, Error> {
        Ok(Self {
            pstr: String::from_utf8(buffer[1..20].to_vec())?,
            info_hash: buffer[28..48].try_into()?,
            peer_id: buffer[48..].try_into()?,
            buffer,
        })
    }
}

impl PartialEq for Handshake {
    fn eq(&self, other: &Self) -> bool {
        self.info_hash == other.info_hash
    }
}

fn handshake_buffer(pstr: &str, info_hash: &[u8; 20], peer_id: &[u8; 20]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(68);
    buffer.push(pstr.len() as u8);
    buffer.extend_from_slice(pstr.as_bytes());
    buffer.extend_from_slice(&[0; 8]);
    buffer.extend_from_slice(info_hash);
    buffer.extend_from_slice(peer_id);
    buffer
}


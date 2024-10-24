pub use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use tokio::net::TcpStream;
use crate::error::Error;

// Generate handshake buffer
fn handshake_buffer(pstr: &str, info_hash: &[u8; 20], peer_id: &[u8; 20]) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(68);
    buffer.push(pstr.len() as u8);
    buffer.extend_from_slice(pstr.as_bytes());
    buffer.extend_from_slice(&[0; 8]); // Reserved bytes
    buffer.extend_from_slice(info_hash);
    buffer.extend_from_slice(peer_id);
    buffer
}

// Handshake struct definition
#[derive(Debug)]
pub struct Handshake {
    pub pstr: String,
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    pub buffer: Vec<u8>,
}

impl Handshake {
    // Create a new handshake
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        let pstr = "BitTorrent protocol".to_string();
        Self { 
            buffer: handshake_buffer(&pstr, &info_hash, &peer_id),
            pstr,
            info_hash,
            peer_id,
        }
    }

    // Create a handshake from a buffer
    pub fn from_buffer(buffer: Vec<u8>) -> Result<Self, Error> {
        Ok(Self {
            pstr: String::from_utf8(buffer[1..20].to_vec())?,
            info_hash: buffer[28..48].try_into()?,
            peer_id: buffer[48..].try_into()?,
            buffer,
        })
    }
}

// Implement equality for Handshake based on info_hash
impl PartialEq for Handshake {
    fn eq(&self, other: &Self) -> bool {
        self.info_hash == other.info_hash
    }
}

// Asynchronous perform_handshake function
pub async fn perform_handshake(stream: &mut TcpStream, handshake_buffer: &[u8]) -> Result<Handshake, Error> {
    stream.write_all(handshake_buffer).await?;
    let mut buffer = [0; 68];
    stream.read_exact(&mut buffer).await?;
    Handshake::from_buffer(buffer.to_vec())
}
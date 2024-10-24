use std::net::Ipv4Addr;
use std::time::Duration;
use std::net::SocketAddrV4;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use reqwest::Client;

mod handshake;
mod message;
mod tracker;
mod error;

use crate::handshake::*;
use crate::message::*;
use crate::tracker::*;
use crate::error::Error;

async fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

fn byte_encode(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|&byte| format!("%{:02X}", byte))
        .collect()
}

async fn send_tracker_request(announce: &str, info_hash: &[u8], peer_id: &str, port: u16, length: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!(
        "{}?info_hash={}&peer_id={}&port={}&uploaded=0&downloaded=0&left={}&compact=1&event=started",
        announce,
        byte_encode(info_hash),
        urlencoding::encode(peer_id),
        port,
        length
    );
    let response = client.get(&url)
        .header("User-Agent", "uTorrent/2210(25110)")
        .send()
        .await?;
    Ok(response.bytes().await?.to_vec())
}

// Структура для общения с пиром
#[derive(Debug)]
struct PeerConnection {
    stream: TcpStream,
    addr: SocketAddrV4
}

impl PeerConnection {
    async fn new(peer_addr: SocketAddrV4, handshake: &Handshake) -> Result<Self, Error> {
        let mut stream = Self::create_stream(&peer_addr).await
            .map_err(|e| Error::PeerConnectionError(e.to_string()))?;
        stream.set_nodelay(true)?;
        let return_handshake = Self::try_handshake(&mut stream, handshake).await?;
        if return_handshake == *handshake {
            Ok(Self { stream, addr: peer_addr })
        } else {
            Err(Error::PeerConnectionError("Info hash does not match!".to_string()))
        }
    }

    async fn create_stream(peer_addr: &SocketAddrV4) -> std::io::Result<TcpStream> {
        tokio::time::timeout(
            Duration::from_secs(5), 
            TcpStream::connect(peer_addr)
        ).await?
    }

    async fn try_handshake(stream: &mut TcpStream, handshake: &Handshake) -> Result<Handshake, Error> {
        tokio::time::timeout(
            Duration::from_secs(5),
            perform_handshake(stream, &handshake.buffer)
        ).await?
    }

    async fn read(&mut self) -> Result<Message, Error> {
        let mut length_buf = [0; 4];
        self.stream.read_exact(&mut length_buf).await?;
        let message_length = u32::from_be_bytes(length_buf);
    
        if message_length > 0 {
            let mut response = vec![0; message_length as usize];
            self.stream.read_exact(&mut response).await?;
            Ok(Message::from_bytes(&response))
        } else {
            Ok(Message::from_bytes(&[0, 4]))
        }
    }

    async fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown().await
    }

    async fn reconnect(mut self, handshake: &Handshake) -> Result<Self, Error> {
        self.shutdown().await.map_err(|_| Error::PeerShutdownConnectionError)?;
        Self::new(self.addr, handshake).await
    }
}

#[tokio::main]
async fn main() {
    // Читаем торрент файл
    let file_data: Vec<u8> = read_file("4326599.torrent").await.unwrap();
    
    let torrent = Torrent::from_file(&file_data).ok_or("").unwrap();
    let info_hash = InfoHash::from_file(&file_data).ok_or("").unwrap();

    let peer_id = gen_peer_id();
    let port: u16 = 6881;
    
    let response = send_tracker_request(
        &torrent.announce,
        info_hash.as_bytes(),
        &peer_id,
        port,
        torrent.info.length.unwrap_or(0),
    ).await.unwrap();

    let tracker_response = serde_bencode::de::from_bytes::<TrackerResponse>(&response).unwrap();

    println!("{:?}", tracker_response);

    let mut socket_addrs = Vec::new();

    let peers = tracker_response.peers.clone();

    for chunk in peers.chunks(6) {
        let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
        let port = ((chunk[4] as u16) << 8) | (chunk[5] as u16);
        let socket_addr = SocketAddrV4::new(ip, port);
        socket_addrs.push(socket_addr);
    }

    let handshake = Handshake::new(
        info_hash.as_bytes().try_into().unwrap(), 
        peer_id.as_bytes().try_into().unwrap()
    );

    for socket_addr in socket_addrs {
        match PeerConnection::new(socket_addr, &handshake).await {
            Ok(mut pc) => {
                println!("Connected to peer: {}", socket_addr);
                if let Ok(m) = pc.read().await {
                    println!("Received message: {:?}", m);
                }
                // If successfully handled, shut down the connection
                let _ = pc.shutdown().await;
            }
            Err(e) => { eprintln!("{}", e); }
        }
    }
}

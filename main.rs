mod message;
mod torrent;
mod error;
mod peer;
mod bencode;
mod piece;
//mod download_manager;

use message::Message;
use tokio::io::AsyncWriteExt;

use crate::torrent::*;
use crate::peer::*;
use crate::error::Error;


#[derive(Debug, Clone)]
pub struct Bitfield {
    data: Vec<u8>
}

impl Bitfield {
    pub fn new(data: Vec<u8>) -> Self {
        Bitfield { data }
    }

    pub fn has_piece(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let offset = index % 8;
        
        if byte_index >= self.data.len() {
            return false;
        }
        (self.data[byte_index] >> (7 - offset)) & 1 != 0
    }

    pub fn set_piece(&mut self, index: usize) {
        let byte_index = index / 8;
        let offset = index % 8;
        if byte_index < self.data.len() {
            self.data[byte_index] |= 1 << (7 - offset);
        }
    }

    pub fn count_pieces(&self) -> usize {
        self.data.iter()
            .map(|byte| byte.count_ones() as usize)
            .sum()
    }

    pub fn len(&self) -> usize {
        self.data.len() * 8
    }

    pub fn to_bits(&self) -> Vec<bool> {
        let mut bits = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            bits.push(self.has_piece(i));
        }
        bits
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub index: usize,
    pub begin: usize,
    pub length: usize,
}

async fn run() {
    let mut payload = Vec::new();
    payload.extend_from_slice(&(3 as u32).to_be_bytes());
    payload.extend_from_slice(&(18604 as u32).to_be_bytes());
    payload.extend_from_slice(&(1266 as u32).to_be_bytes());
    println!("{:?}", payload);

    let mut tc = TorrentClient::from_path("Lana Del Rey.torrent").await.unwrap();

    //tc.torrent.info.pieces = serde_bytes::ByteBuf::new();

    //println!("{:#?}", tc);
    //panic!();

    let connection_futures: Vec<_> = tc.torrent.peers_addr
        .iter()
        .map(|&peer_addr| {
            let handshake = tc.handshake.clone();
            async move {
                if let Ok(conn) = PeerConnection::new(peer_addr, &handshake).await {
                    println!("Connected to peer: {}", peer_addr);
                    return Some(conn);
                }
                eprintln!("Failed to connect to peer: {}", peer_addr);
                None
            }
        })
        .collect();

    let mut peer_connections: Vec<PeerConnection> = futures::future::join_all(connection_futures)
        .await
        .into_iter()
        .filter_map(|v| v)
        .collect();
    
    for peer in peer_connections.iter_mut() {
        if let Ok(receive_message) = peer.receive().await {
            let bitfield_data = Bitfield::new(receive_message.payload);
            //println!("{:?}", bitfield_data.to_bits());
        }
        let mess = Message::new(
            message::MessageTag::Interested, 
            Vec::new()
        );
        peer.send(mess).await.unwrap();
        let mut payload = Vec::new();
        payload.extend_from_slice(&(3 as u32).to_be_bytes());
        payload.extend_from_slice(&(18604 as u32).to_be_bytes());
        payload.extend_from_slice(&(1266 as u32).to_be_bytes());
        let mess = Message::new(
            message::MessageTag::Request, 
            payload
        );
        peer.send(mess).await.unwrap();
        while  true {
            if let Ok(receive_message) = peer.receive().await {
                println!("{:?}", receive_message);
                if receive_message.tag == message::MessageTag::Piece {
                    let block_data = &receive_message.payload[8..];
                    let mut file = tokio::fs::File::create("Folder.auCDtect.txt").await.unwrap();
                    if block_data.len() != 1266 {
                        println!("Неверный размер блока! {}", block_data.len());
                    }
                    file.write_all(&block_data).await.unwrap();
                    file.flush().await.unwrap();
                    break;
                }
            }
        }
    }

    for pc in peer_connections.iter_mut() {
        let _ = pc.shutdown().await;
    }
    println!("{:#?}", peer_connections);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(run());
    Ok(())
}

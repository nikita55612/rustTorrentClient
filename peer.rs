use std::time::Duration;
use std::net::SocketAddrV4;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::net::SocketAddr;

use crate::message::*;
use crate::error::Error;
use crate::torrent::Handshake;


#[derive(Debug)]
pub struct PeerConnection {
    stream: TcpStream
}

impl PeerConnection {
    pub async fn new(peer_addr: SocketAddrV4, handshake: &Handshake) -> Result<Self, Error> {
        let mut stream = Self::create_stream(&peer_addr).await
            .map_err(|e| Error::PeerConnectionError(e.to_string()))?;
        stream.set_nodelay(true)?;
        let return_handshake = Self::try_handshake(&mut stream, handshake).await?;
        if return_handshake == *handshake {
            Ok(Self { stream })
        } else {
            Err(Error::PeerConnectionError("Info hash does not match!".to_string()))
        }
    }

    async fn create_stream(peer_addr: &SocketAddrV4) -> std::io::Result<TcpStream> {
        tokio::time::timeout(
            Duration::from_secs(4), 
            TcpStream::connect(peer_addr)
        ).await?
    }

    async fn try_handshake(stream: &mut TcpStream, handshake: &Handshake) -> Result<Handshake, Error> {
        tokio::time::timeout(
            Duration::from_secs(4),
            perform_handshake(stream, &handshake.buffer)
        ).await?
    }

    pub async fn receive(&mut self) -> Result<Message, Error> {
        let mut length_buf = [0; 4];
        tokio::time::timeout(
            Duration::from_secs(2),
            self.stream.read_exact(&mut length_buf)
        ).await??;
        let message_length = u32::from_be_bytes(length_buf);
        if message_length > 0 {
            let mut response = vec![0; message_length as usize];
            tokio::time::timeout(
                Duration::from_secs(2),
                self.stream.read_exact(&mut response)
            ).await??;
            Ok(Message::from_bytes(&response))
        } else {
            Ok(Message::from_bytes(&[0, 4]))
        }
    }

    pub async fn send(&mut self, message: Message) -> Result<(), Error> {
        tokio::time::timeout(
            Duration::from_secs(2),
            self.stream.write_all(&message.to_buffer())
        ).await??;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown().await
    }

    pub async fn reconnect(mut self, handshake: &Handshake) -> Result<Self, Error> {
        let peer_addr = self.stream.peer_addr()?;
        let peer_addr_v4 = match peer_addr {
            SocketAddr::V4(addr) => addr,
            _ => return Err(Error::AnyError("UnsupportedAddressType".to_string())),
        };
        self.shutdown().await.map_err(|_| Error::PeerShutdownConnectionError)?;
        Self::new(peer_addr_v4, handshake).await
    }
}

pub async fn perform_handshake(stream: &mut TcpStream, handshake_buffer: &[u8]) -> Result<Handshake, Error> {
    stream.write_all(handshake_buffer).await?;
    let mut buffer = [0; 68];
    stream.read_exact(&mut buffer).await?;
    Handshake::from_buffer(buffer.to_vec())
}

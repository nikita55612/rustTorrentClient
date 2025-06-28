use std::net::SocketAddrV4;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use crate::error::Result;
use crate::proto::handshake::Handshake;

const TIMEOUT: Duration = Duration::from_secs(5);

pub struct PeerConn {
    stream: TcpStream,
}

impl PeerConn {
    pub async fn connect(addr: &SocketAddrV4) -> Result<Self> {
        let stream = timeout(TIMEOUT, TcpStream::connect(addr)).await??;
        // let (reader, writer) = tokio::io::split(stream);

        // let mut reader = BufReader::new(reader);
        // let mut writer = BufWriter::new(writer);

        Ok(Self { stream })
    }

    async fn background(&mut self) {}

    pub async fn handshake(&mut self, h: &Handshake) -> Result<Handshake> {
        self.send(h.bytes()).await?;
        let data = self.receive(68).await?;

        Ok(Handshake::new(data.as_slice().try_into()?))
    }

    pub async fn send(&mut self, data: &[u8]) -> Result<()> {
        timeout(TIMEOUT, self.stream.write_all(data)).await??;

        Ok(())
    }

    pub async fn receive(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0u8; n];

        let n = timeout(TIMEOUT, self.stream.read(&mut buf)).await??;
        buf.truncate(n);

        Ok(buf)
    }
}

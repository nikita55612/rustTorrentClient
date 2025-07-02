use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::ErrorKind::WouldBlock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, timeout, Duration};

use crate::error::{Error, Result};
use crate::peer::message::Message;

const CONN_TIMEOUT: Duration = Duration::from_secs(7);
const WRITE_TIMEOUT: Duration = Duration::from_secs(5);
const PING_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Debug)]
pub struct Conn {
    addr: SocketAddr,
    writer: Arc<Mutex<OwnedWriteHalf>>,
    done_tx: mpsc::Sender<()>,
}

impl Drop for Conn {
    fn drop(&mut self) {
        let done_tx = self.done_tx.clone();
        tokio::spawn(async move {
            let _ = done_tx.send(()).await;
        });
    }
}

impl Conn {
    pub async fn from_stream(stream: TcpStream) -> Result<(Self, mpsc::Receiver<Message>)> {
        let peer_addr = stream.peer_addr()?;
        let (mut reader, writer) = stream.into_split();

        let writer = Arc::new(Mutex::new(writer));
        let (done_tx, mut done_rx) = mpsc::channel(1);

        let ping_handle = {
            let writer = writer.clone();
            let done_tx = done_tx.clone();
            tokio::spawn(async move {
                loop {
                    let mut guard = writer.lock().await;
                    if guard
                        .write_all(&Message::KeepAlive.to_bytes())
                        .await
                        .is_err()
                    {
                        break;
                    };
                    sleep(PING_INTERVAL).await;
                }
                let _ = done_tx.send(());
            })
        };

        let (tx, rx) = mpsc::channel::<Message>(1024);

        let reader_handle = {
            let done_tx = done_tx.clone();
            tokio::spawn(async move {
                let mut buf = [0u8; 4096];
                'outer: loop {
                    match reader.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(n) => {
                            let mut s = 0;
                            while s < n {
                                let msg = Message::from_bytes(&buf[s..n]);
                                s += msg.len();
                                if tx.send(msg).await.is_err() {
                                    break 'outer;
                                }
                            }
                        }
                        Err(ref e) if e.kind() == WouldBlock => {
                            continue;
                        }
                        Err(_) => break,
                    }
                }
                let _ = done_tx.send(());
            })
        };

        tokio::spawn(async move {
            done_rx.recv().await;
            done_rx.close();

            ping_handle.abort();
            reader_handle.abort();

            let _ = tokio::join!(ping_handle, reader_handle);
        });

        Ok((
            Self {
                addr: peer_addr,
                writer: writer,
                done_tx: done_tx,
            },
            rx,
        ))
    }

    pub async fn connect(addr: impl ToSocketAddrs) -> Result<(Self, mpsc::Receiver<Message>)> {
        let stream = timeout(CONN_TIMEOUT, TcpStream::connect(addr)).await??;
        Self::from_stream(stream).await
    }

    pub async fn reconnect(&mut self) -> Result<mpsc::Receiver<Message>> {
        self.close().await;
        let (conn, rx) = Self::connect(&self.addr).await?;
        *self = conn;
        Ok(rx)
    }

    pub async fn is_closed(&self) -> bool {
        self.done_tx.is_closed()
    }

    pub async fn send(&self, msg: &Message) -> Result<()> {
        if self.is_closed().await {
            return Err(Error::Custom("Connection is closed".into()));
        }

        let mut guard = self.writer.lock().await;
        timeout(WRITE_TIMEOUT, guard.write_all(&msg.to_bytes())).await??;

        Ok(())
    }

    pub async fn close(&self) {
        if !self.is_closed().await {
            let _ = self.done_tx.send(()).await;
        }
    }
}

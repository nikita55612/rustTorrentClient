use std::fmt::Debug;
use std::net::SocketAddrV4;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::ErrorKind::WouldBlock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::{timeout, Duration};

use crate::error::{Error, Result};
use crate::proto::message::Message;

const CONN_TIMEOUT: Duration = Duration::from_secs(8);
const WRITE_TIMEOUT: Duration = Duration::from_secs(4);
const PING_INTERVAL: Duration = Duration::from_secs(14);

struct ConnInner {
    writer: Option<OwnedWriteHalf>,
    ping_handle: Option<JoinHandle<()>>,
    reader_handle: Option<JoinHandle<()>>,
    is_open: bool,
}

pub struct Conn {
    inner: Arc<Mutex<ConnInner>>,
    done_flag: Arc<AtomicBool>,
}

impl Debug for Conn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conn")
    }
}

impl Drop for Conn {
    fn drop(&mut self) {
        let inner = self.inner.clone();
        tokio::spawn(async move {
            let mut guard = inner.lock().await;
            guard.ping_handle.as_mut().map(|jh| jh.abort());
            guard.reader_handle.as_mut().map(|jh| jh.abort());
            if let Some(writer) = guard.writer.as_mut() {
                let _ = writer.shutdown().await;
            }
        });
    }
}

impl Conn {
    pub async fn connect(addr: &SocketAddrV4) -> Result<(Self, mpsc::Receiver<Message>)> {
        let stream = timeout(CONN_TIMEOUT, TcpStream::connect(addr)).await??;
        let (reader, writer) = stream.into_split();
        let (tx, rx) = mpsc::channel::<Message>(1024);

        let inner = Arc::new(Mutex::new(ConnInner {
            writer: Some(writer),
            ping_handle: None,
            reader_handle: None,
            is_open: false,
        }));

        let done_flag = Arc::new(AtomicBool::new(false));

        let ping_handle = {
            let inner = inner.clone();
            let done_flag = done_flag.clone();
            tokio::spawn(async move {
                Self::ping_loop(inner, done_flag).await;
            })
        };

        let reader_handle = {
            let done_flag = done_flag.clone();
            tokio::spawn(async move {
                Self::reader_loop(reader, tx, done_flag).await;
            })
        };

        {
            let mut guard = inner.lock().await;
            guard.ping_handle = Some(ping_handle);
            guard.reader_handle = Some(reader_handle);
            guard.is_open = true;
        }

        Ok((
            Conn {
                inner: inner,
                done_flag: done_flag,
            },
            rx,
        ))
    }

    pub async fn reconnect(&self, addr: &SocketAddrV4) -> Result<mpsc::Receiver<Message>> {
        self.close().await;

        let stream = timeout(CONN_TIMEOUT, TcpStream::connect(addr)).await??;
        let (reader, writer) = stream.into_split();
        let (tx, rx) = mpsc::channel::<Message>(1024);

        let done_flag = Arc::new(AtomicBool::new(false));

        let ping_handle = {
            let inner = self.inner.clone();
            let done_flag = done_flag.clone();
            tokio::spawn(async move {
                Self::ping_loop(inner, done_flag).await;
            })
        };

        let reader_handle = tokio::spawn(async move {
            Self::reader_loop(reader, tx, done_flag.clone()).await;
        });

        let mut guard = self.inner.lock().await;
        guard.writer = Some(writer);
        guard.ping_handle = Some(ping_handle);
        guard.reader_handle = Some(reader_handle);
        guard.is_open = true;

        Ok(rx)
    }

    pub async fn is_open(&self) -> bool {
        !self.done_flag.load(Ordering::Acquire) && self.inner.lock().await.is_open
    }

    async fn ping_loop(inner: Arc<Mutex<ConnInner>>, done_flag: Arc<AtomicBool>) {
        while !done_flag.load(Ordering::Acquire) {
            {
                let mut guard = inner.lock().await;
                if let Some(writer) = guard.writer.as_mut() {
                    if writer
                        .write_all(&Message::KeepAlive.to_bytes())
                        .await
                        .is_err()
                    {
                        break;
                    }
                } else {
                    break;
                }
            }
            tokio::time::sleep(PING_INTERVAL).await;
        }
        done_flag.store(true, Ordering::Release);
        let mut guard = inner.lock().await;
        if let Some(ref jh) = guard.reader_handle {
            if !jh.is_finished() {
                jh.abort();
            }
        }
        guard.is_open = false;
    }

    async fn reader_loop(
        mut reader: OwnedReadHalf,
        tx: mpsc::Sender<Message>,
        done_flag: Arc<AtomicBool>,
    ) {
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf).await {
                Ok(0) => break,
                Ok(n) => {
                    let mut s = 0;
                    while s < n {
                        let msg = Message::from_bytes(&buf[s..n]);
                        s += msg.len();
                        if tx.send(msg).await.is_err() {
                            break;
                        }
                    }
                }
                Err(ref e) if e.kind() == WouldBlock => {
                    continue;
                }
                Err(_) => break,
            }
        }
        done_flag.store(true, Ordering::Release);
    }

    pub async fn send(&self, data: &[u8]) -> Result<()> {
        let mut guard = self.inner.lock().await;
        if !guard.is_open {
            return Err(Error::Custom("Connection is closed".into()));
        }

        let writer = guard
            .writer
            .as_mut()
            .ok_or_else(|| Error::Custom("Writer not available".into()))?;
        timeout(WRITE_TIMEOUT, writer.write_all(data)).await??;

        Ok(())
    }

    pub async fn lock(&self) -> JoinHandle<()> {
        let inner = self.inner.clone();
        tokio::spawn(async move {
            let _ = inner.lock().await;
            loop {
                tokio::time::sleep(Duration::from_secs(99999)).await;
            }
        })
    }

    pub async fn close(&self) {
        let mut guard = self.inner.lock().await;
        if !guard.is_open {
            return;
        }
        if let Some(ref mut jh) = guard.ping_handle {
            if !jh.is_finished() {
                jh.abort();
            }
            let _ = jh.await;
        }
        if let Some(ref mut jh) = guard.reader_handle {
            if !jh.is_finished() {
                jh.abort();
            }
            let _ = jh.await;
        }
        if let Some(writer) = guard.writer.as_mut() {
            let _ = writer.shutdown().await;
        }

        guard.writer = None;
        guard.ping_handle = None;
        guard.reader_handle = None;
        guard.is_open = false;
    }
}

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{mpsc::Sender, Mutex},
    task::JoinHandle,
};

use crate::{
    error::Result,
    torrent::{infohash::InfoHash, Torrent},
};

const DEFAULT_UDP_PORT: u16 = 6881;
const DEFAULT_TCP_PORT: u16 = 6881;

type UdpTrackersRedirect = Arc<Mutex<HashMap<SocketAddr, Sender<Vec<u8>>>>>;
type Torrents = Arc<Mutex<HashMap<InfoHash, Torrent>>>;

pub struct Session {
    udp_socket: Arc<UdpSocket>,
    udp_task: JoinHandle<()>,
    udp_trackers_redirect: UdpTrackersRedirect,
    torrents: Torrents,
}

// Однако асинхронный мьютекс обходится дороже обычного, и обычно лучше использовать один из двух других подходов.

impl Session {
    pub async fn new() -> Result<Self> {
        let tcp_listener = TcpListener::bind(("0.0.0.0", DEFAULT_TCP_PORT)).await?;
        let udp_socket = Arc::new(UdpSocket::bind(("0.0.0.0", DEFAULT_UDP_PORT)).await?);
        let udp_trackers_redirect: UdpTrackersRedirect = Arc::new(Mutex::new(HashMap::new()));
        let torrents: Torrents = Arc::new(Mutex::new(HashMap::new()));

        //
        let udp_task = {
            let udp_socket = udp_socket.clone();
            let udp_trackers_redirect = udp_trackers_redirect.clone();

            tokio::spawn(async move {
                let mut buf = [0u8; 2048];

                while let Ok((n, addr)) = udp_socket.recv_from(&mut buf).await {
                    println!("{:?} bytes received from {:?}", n, addr);

                    if buf.starts_with(b"d1:") {
                        // DHT bencoded message
                    } else {
                        let mut udp_trackers_redirect_guard = udp_trackers_redirect.lock().await;
                        if let Some(s) = udp_trackers_redirect_guard.get(&addr) {
                            if s.send(buf[..n].to_vec()).await.is_err() {
                                udp_trackers_redirect_guard.remove(&addr);
                            }
                        }
                        drop(udp_trackers_redirect_guard)
                    }
                }
                // udp_socket.take_error()
            })
        };

        Ok(Self {
            udp_socket,
            udp_task,
            udp_trackers_redirect,
            torrents,
        })
    }

    pub async fn run(&self) {}

    pub async fn insert_trackers_redirect(&mut self, addr: SocketAddr, sender: Sender<Vec<u8>>) {
        self.udp_trackers_redirect.lock().await.insert(addr, sender);
    }

    pub async fn remove_trackers_redirect(&mut self, addr: &SocketAddr) -> Option<Sender<Vec<u8>>> {
        self.udp_trackers_redirect.lock().await.remove(addr)
    }
}

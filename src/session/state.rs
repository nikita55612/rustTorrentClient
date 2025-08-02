use crate::{
    error::{Error, Result},
    session::{Bep15ResponseRouter, DhtResponseRouter, SessionAlert},
    torrent::{TorrentCommand, TorrentID},
};
use std::collections::BTreeMap;
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

const DEFAULT_UDP_PORT: u16 = 6881;
const DEFAULT_TCP_PORT: u16 = 6881;

type TorrentsCmd = BTreeMap<TorrentID, Sender<TorrentCommand>>;

pub struct SessionState {
    pub udp_socket: UdpSocket,
    pub tcp_listener: TcpListener,
    pub dht_router: Mutex<DhtResponseRouter>,
    pub bep15_router: Mutex<Bep15ResponseRouter>,
    torrents_cmd: Mutex<TorrentsCmd>,
    alert_tx: Sender<SessionAlert>,
}

impl SessionState {
    pub async fn init() -> Result<(Self, Receiver<SessionAlert>)> {
        let udp_socket = UdpSocket::bind(("0.0.0.0", DEFAULT_UDP_PORT)).await?;
        let tcp_listener = TcpListener::bind(("0.0.0.0", DEFAULT_TCP_PORT)).await?;

        let dht_router = Mutex::new(DhtResponseRouter::new());
        let bep15_router = Mutex::new(Bep15ResponseRouter::new());

        let torrents_cmd = Mutex::new(TorrentsCmd::new());
        let (alert_tx, alert_rx) = channel::<SessionAlert>(4);

        Ok((
            Self {
                udp_socket,
                tcp_listener,
                dht_router,
                bep15_router,
                alert_tx,
                torrents_cmd,
            },
            alert_rx,
        ))
    }

    pub async fn send_alert(&self, alert: SessionAlert) -> Result<()> {
        self.alert_tx
            .send(alert)
            .await
            .map_err(|e| Error::SendSessionAlert(e.0))
    }

    pub async fn send_to_torrent_cmd(
        &self,
        torrent_id: &TorrentID,
        command: TorrentCommand,
    ) -> Option<Result<()>> {
        if let Some(cmd) = self.torrents_cmd.lock().await.get(torrent_id) {
            Some(
                cmd.send(command)
                    .await
                    .map_err(|e| Error::SendToTorrentCmd(e.0)),
            )
        } else {
            None
        }
    }
}

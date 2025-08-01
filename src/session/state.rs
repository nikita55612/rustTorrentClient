use crate::{
    error::Result,
    session::{Bep15ResponseRouter, DhtResponseRouter, SessionAlert},
};
use tokio::{
    net::{TcpListener, UdpSocket},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
};

const DEFAULT_UDP_PORT: u16 = 6881;
const DEFAULT_TCP_PORT: u16 = 6881;

pub struct SessionState {
    pub udp_socket: UdpSocket,
    pub tcp_listener: TcpListener,
    pub dht_router: Mutex<DhtResponseRouter>,
    pub bep15_router: Mutex<Bep15ResponseRouter>,
    pub alert_tx: Sender<SessionAlert>,
}

impl SessionState {
    pub async fn init() -> Result<(Self, Receiver<SessionAlert>)> {
        let udp_socket = UdpSocket::bind(("0.0.0.0", DEFAULT_UDP_PORT)).await?;
        let tcp_listener = TcpListener::bind(("0.0.0.0", DEFAULT_TCP_PORT)).await?;

        let dht_router = Mutex::new(DhtResponseRouter::new());
        let bep15_router = Mutex::new(Bep15ResponseRouter::new());

        let (alert_tx, alert_rx) = channel::<SessionAlert>(4);

        Ok((
            Self {
                udp_socket,
                tcp_listener,
                dht_router,
                bep15_router,
                alert_tx,
            },
            alert_rx,
        ))
    }
}

use crate::{
    proto::{bep15::Bep15Response, dht::KrpcMessage},
    session::state::SessionState,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    sync::mpsc::{channel, Sender},
    task::JoinHandle,
};

pub async fn spawn_udp_listener(state: Arc<SessionState>) -> JoinHandle<()> {
    let udp_packet_handler_tx = spawn_udp_packet_handler(state.clone()).await;

    tokio::spawn(async move {
        let mut buf = [0u8; 2048];

        while let Ok((n, addr)) = state.udp_socket.recv_from(&mut buf).await {
            let _ = udp_packet_handler_tx.send((addr, buf[..n].to_vec())).await;
        }
    })
}

#[derive(Debug, Copy, Clone)]
enum Protocol {
    DHT,
    Tracker,
    // Peer,
    Unknown,
}

async fn spawn_udp_packet_handler(state: Arc<SessionState>) -> Sender<(SocketAddr, Vec<u8>)> {
    let (tx, mut rx) = channel::<(SocketAddr, Vec<u8>)>(4);

    tokio::spawn(async move {
        while let Some((addr, packet)) = rx.recv().await {
            match identify_udp_protocol(&packet) {
                Protocol::DHT => {
                    if let Ok(msg) = KrpcMessage::from_bytes(&packet) {
                        let _ = state
                            .dht_router
                            .lock()
                            .await
                            .do_redirect(&addr, &msg.transaction_id().unwrap_or_default(), msg)
                            .await;
                    }
                }
                Protocol::Tracker => {
                    if let Ok(msg) = Bep15Response::from_bytes(&packet) {
                        let _ = state
                            .bep15_router
                            .lock()
                            .await
                            .do_redirect(&addr, &msg.transaction_id(), msg)
                            .await;
                    }
                }
                Protocol::Unknown => {}
            }
        }
    });

    tx
}

#[inline]
fn identify_udp_protocol(packet: &[u8]) -> Protocol {
    if packet.len() < 4 {
        return Protocol::Unknown;
    }
    // b"d" = [100]
    if packet[0] == 100 {
        return Protocol::DHT;
    } else {
        match i32::from_be_bytes(packet[..4].try_into().unwrap()) {
            0..=3 => Protocol::Tracker,
            _ => Protocol::Unknown,
        }
    }
}

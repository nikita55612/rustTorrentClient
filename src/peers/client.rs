// // 🔄 Полная последовательность
// // TCP connect

// // Отправка handshake

// // Получение handshake

// // Получение bitfield

// // Отправка interested

// // Получение unchoke

// // Отправка request

// // Получение piece

// use std::{
//     collections::HashSet,
//     net::SocketAddrV4,
//     ops::{Deref, DerefMut},
// };

// use crate::{
//     error::Result,
//     peer::{conn::Conn, peer::Peer, util::socket_addr_from_bytes},
// };

// use tokio::sync::mpsc;

// #[derive(Debug)]
// pub struct PeersClient {
//     peers: Vec<Peer>,
// }

// impl PeersClient {
//     pub fn new() -> Self {
//         let (tx, rx) = mpsc::channel(100);

//         let jh = tokio::spawn(future);
//     }

//     pub fn x() {}
// }

// impl Deref for Manager {
//     type Target = Vec<Peer>;

//     fn deref(&self) -> &Self::Target {
//         &self.peers
//     }
// }

// impl DerefMut for Manager {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.peers
//     }
// }

// impl Manager {
//     pub fn new() -> Self {
//         Self { peers: Vec::new() }
//     }

//     pub fn from_bytes(bytes: &[u8]) -> Self {
//         let peers = bytes
//             .chunks(6)
//             .filter_map(|v| v.try_into().ok().map(Peer::from_bytes))
//             .collect();

//         Self { peers: peers }
//     }

//     pub fn update_peers_from_bytes(&mut self, bytes: &[u8]) -> () {
//         let new_addrs: HashSet<SocketAddrV4> = bytes
//             .chunks(6)
//             .filter_map(|chunk| chunk.try_into().ok().map(socket_addr_from_bytes))
//             .collect();

//         let mut remaining_peers = Vec::new();
//         for peer in self.peers.drain(..) {
//             if new_addrs.contains(&peer.addr) {
//                 remaining_peers.push(peer);
//             } else if let Some(conn) = peer.conn {
//                 tokio::spawn(async move { conn.close().await });
//             }
//         }

//         self.peers = remaining_peers;

//         let existing_addrs: HashSet<_> = self.peers.iter().map(|p| p.addr).collect();
//         for addr in new_addrs {
//             if !existing_addrs.contains(&addr) {
//                 self.peers.push(Peer::new(addr));
//             }
//         }
//     }

//     pub fn get_peer_from_addr(&self, addr: &SocketAddrV4) -> Option<&Peer> {
//         self.peers.iter().find(|p| &p.addr == addr)
//     }

//     pub async fn total_connections(&self) -> usize {
//         let mut res = 0usize;
//         for peer in self.peers.iter() {
//             if let Some(ref conn) = peer.conn {
//                 res += !conn.is_closed().await as usize;
//             }
//         }
//         res
//     }
// }

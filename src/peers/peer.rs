// use std::net::SocketAddrV4;
// use tokio::sync::mpsc::Receiver;

// use crate::{
//     error::Result,
//     peer::message::{BitField, Handshake, Message},
//     peer::{conn::Conn, util::socket_addr_from_bytes},
// };

// #[derive(Debug)]
// pub struct Peer {
//     pub conn: Conn,
//     pub receiver: Option<Receiver<Message>>,
//     pub handshake: Option<Handshake>,
//     pub bitfield: Option<BitField>,
//     pub choked: bool,
//     pub rejected: bool,
//     // pub update time
//     // am_choking: this client is choking the peer
//     // am_interested: this client is interested in the peer
//     // peer_choking: peer is choking this client
//     // peer_interested: peer is interested in this client
// }

// impl PartialEq for Peer {
//     fn eq(&self, other: &Self) -> bool {
//         self.conn.peer_addr() == other.conn.peer_addr()
//     }
// }

// impl Eq for Peer {}

// // 🔄 Полная последовательность
// // TCP connect

// // Отправка handshake

// // Получение handshake

// // Получение bitfield

// // Отправка interested

// // Получение unchoke

// // Отправка request

// // Получение piece

// impl Peer {
//     pub fn from_conn(conn: Conn) -> Self {
//         Self {
//             conn: conn,
//             receiver: None,
//             handshake: None,
//             bitfield: None,
//             choked: false,
//             rejected: false,
//         }
//     }

//     pub async fn handshake() {}

//     pub async fn x(&mut self) {
//         if self.conn.is_closed().await {
//             if let Ok(receiver) = self.conn.reconnect().await {
//                 self.receiver = Some(receiver);
//                 return;
//             }
//         }
//         if self.handshake.is_none() {
//             return;
//         }
//     }

//     pub async fn read_all(&mut self) -> Vec<Message> {
//         let mut messages = Vec::new();
//         if let Some(ref mut receiver) = self.receiver {
//             while !receiver.is_empty() {
//                 if let Some(msg) = receiver.recv().await {
//                     messages.push(msg);
//                 }
//             }
//         }
//         messages
//     }

//     pub fn take_receiver(&mut self) -> Option<Receiver<Message>> {
//         self.receiver.take()
//     }

//     pub fn take_conn(&mut self) -> Option<Conn> {
//         self.conn.take()
//     }

//     pub async fn send(&self, msg: &Message) -> Result<()> {
//         if let Some(ref conn) = self.conn {
//             conn.send(msg).await?
//         }
//         Ok(())
//     }

//     pub async fn try_connect(&mut self) -> Result<()> {
//         if let Some(ref mut conn) = self.conn {
//             if !conn.is_closed().await {
//                 return Ok(());
//             }
//             match conn.reconnect().await {
//                 Ok(receiver) => {
//                     self.receiver = Some(receiver);
//                     return Ok(());
//                 }
//                 Err(e) => {
//                     self.conn_attempts += 1;
//                     return Err(e);
//                 }
//             }
//         }
//         match Conn::connect(&self.addr).await {
//             Ok((conn, receiver)) => {
//                 self.conn = Some(conn);
//                 self.receiver = Some(receiver);
//                 Ok(())
//             }
//             Err(e) => {
//                 self.conn_attempts += 1;
//                 Err(e)
//             }
//         }
//     }
// }

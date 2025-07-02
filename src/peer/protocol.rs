// use std::net::SocketAddrV4;
// use tokio::sync::mpsc::Receiver;

// use crate::{
//     error::Result,
//     peers::{conn::Conn, util::socket_addr_from_bytes},
//     proto::{BitField, Handshake, Message},
// };

// #[derive(Debug)]
// pub struct PeerWire {
//     pub addr: SocketAddrV4,
//     pub conn: Option<Conn>,
//     pub conn_attempts: usize,
//     pub receiver: Option<Receiver<Message>>,
//     pub handshake: Option<Handshake>,
//     pub bitfield: Option<BitField>,
//     pub choked: bool,
//     pub rejected: bool,
// }

// impl PartialEq for Peer {
//     fn eq(&self, other: &Self) -> bool {
//         self.addr == other.addr
//     }
// }

// impl Eq for Peer {}

// impl Peer {
//     pub fn new(addr: SocketAddrV4) -> Self {
//         Self {
//             addr: addr,
//             conn: None,
//             conn_attempts: 0,
//             receiver: None,
//             handshake: None,
//             bitfield: None,
//             choked: false,
//             rejected: false,
//         }
//     }

//     pub fn from_bytes(bytes: &[u8; 6]) -> Self {
//         Self::new(socket_addr_from_bytes(bytes))
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

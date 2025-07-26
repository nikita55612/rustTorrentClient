use super::bitfield::BitField;
use super::constants::*;
use super::handshake::Handshake;
use super::piece::Piece;
use super::request::Request;
use crate::proto::bep10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Empty,
    Invalid(usize),
    KeepAlive,
    Choke,
    UnChoke,
    Interested,
    NotInterested,
    Have(u32),
    BitField(BitField),
    Request(Request),
    Cancel(Request),
    Piece(Piece),
    Handshake(Handshake),
    Port(u16),
    Extension(bep10::ExtensionMessage),
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        match n {
            0 => return Self::Empty,
            x if x < 4 => return Self::Invalid(n),
            _ => (),
        }

        if bytes[..4] == KEEP_ALIVE_MESS {
            return Self::KeepAlive;
        }

        if n < 5 {
            return Self::Invalid(n);
        }

        if n >= HANDSHAKE_LEN && bytes[..5] == HANDSHAKE_PREFIX {
            let bytes: [u8; HANDSHAKE_LEN] = bytes[..HANDSHAKE_LEN].try_into().unwrap();
            return Self::Handshake(Handshake(bytes));
        }

        let len = u32::from_be_bytes(bytes[..4].try_into().unwrap()) as usize;
        let end = len + 4;
        if end > n {
            return Self::Invalid(n);
        }

        let message_id = bytes[4];
        let payload = &bytes[5..end];

        match message_id {
            CHOKE_MESS_ID if len == 1 => Self::Choke,
            UNCHOKE_MESS_ID if len == 1 => Self::UnChoke,
            INTERESTED_MESS_ID if len == 1 => Self::Interested,
            NOT_INTERESTED_MESS_ID if len == 1 => Self::NotInterested,
            HAVE_MESS_ID if len == HAVE_LEN => {
                Self::Have(u32::from_be_bytes(payload.try_into().unwrap()))
            }
            BITFIELD_MESS_ID if len > 1 => Self::BitField(BitField(payload.into())),
            REQUEST_MESS_ID if len == REQUEST_LEN => {
                let bytes: [u8; REQUEST_LEN - 1] = payload.try_into().unwrap();
                Self::Request(Request::from_bytes(bytes))
            }
            CANCEL_MESS_ID if len == REQUEST_LEN => {
                let bytes: [u8; REQUEST_LEN - 1] = payload.try_into().unwrap();
                Self::Cancel(Request::from_bytes(bytes))
            }
            PIECE_MESS_ID if len > 8 => Self::Piece(Piece::from_bytes(payload)),
            PORT_MESS_ID if len == PORT_LEN => {
                Self::Port(u16::from_be_bytes(payload.try_into().unwrap()))
            }
            EXTENSION_MESS_ID if len > 6 => {
                Self::Extension(bep10::ExtensionMessage::from_bytes(&bytes[..end]))
            }
            _ => Message::Invalid(n),
        }
    }

    pub fn to_vec_of_bytes(&self) -> Vec<u8> {
        match self {
            Self::KeepAlive => KEEP_ALIVE_MESS.to_vec(),
            Self::Handshake(h) => h.to_vec(),
            Self::Choke => CHOKE_MESS.to_vec(),
            Self::UnChoke => UNCHOKE_MESS.to_vec(),
            Self::Interested => INTERESTED_MESS.to_vec(),
            Self::NotInterested => NOT_INTERESTED_MESS.to_vec(),
            Self::Have(i) => {
                let mut buf = [0u8; 4 + HAVE_LEN];
                buf[..5].copy_from_slice(&[0, 0, 0, HAVE_LEN as u8, HAVE_MESS_ID]);
                buf[5..].copy_from_slice(&i.to_be_bytes());
                buf.to_vec()
            }
            Self::BitField(b) => {
                let len = 1 + b.len();
                let mut buf = Vec::with_capacity(4 + len);
                buf.extend_from_slice(&(len as u32).to_be_bytes());
                buf.push(BITFIELD_MESS_ID);
                buf.extend_from_slice(b);
                buf
            }
            Self::Request(r) | Self::Cancel(r) => {
                let id = if let Self::Request(_) = self {
                    REQUEST_MESS_ID
                } else {
                    CANCEL_MESS_ID
                };
                let mut buf = [0u8; 4 + REQUEST_LEN];
                buf[..5].copy_from_slice(&[0, 0, 0, REQUEST_LEN as u8, id]);
                buf[5..HAVE_LEN].copy_from_slice(&r.index.to_be_bytes());
                buf[HAVE_LEN..13].copy_from_slice(&r.begin.to_be_bytes());
                buf[13..].copy_from_slice(&r.length.to_be_bytes());
                buf.to_vec()
            }
            Self::Piece(p) => {
                let len = 9 + p.block.len();
                let mut buf = Vec::with_capacity(4 + len);
                buf.extend_from_slice(&(len as u32).to_be_bytes());
                buf.push(PIECE_MESS_ID);
                buf.extend_from_slice(&p.index.to_be_bytes());
                buf.extend_from_slice(&p.begin.to_be_bytes());
                buf.extend_from_slice(&p.block);
                buf
            }
            Self::Port(p) => {
                let mut buf = [0u8; 4 + PORT_LEN];
                buf[..5].copy_from_slice(&[0, 0, 0, PORT_LEN as u8, PORT_MESS_ID]);
                buf[5..].copy_from_slice(&p.to_be_bytes());
                buf.to_vec()
            }
            _ => Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Invalid(n) => *n,
            Self::KeepAlive => 4,
            Self::Handshake(_) => HANDSHAKE_LEN,
            Self::Choke | Self::UnChoke | Self::Interested | Self::NotInterested => 4 + 1,
            Self::Have(_) => 4 + HAVE_LEN,
            Self::BitField(b) => 4 + 1 + b.len(),
            Self::Request(_) | Self::Cancel(_) => 4 + REQUEST_LEN,
            Self::Piece(p) => 4 + 1 + 8 + p.block.len(),
            Self::Port(_) => 4 + PORT_LEN,
            Self::Extension(e) => e.len(),
        }
    }
}

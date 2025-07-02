use crate::peer::message::bitfield::BitField;
use crate::peer::message::constants::*;
use crate::peer::message::handshake::Handshake;
use crate::peer::message::piece::Piece;
use crate::peer::message::request::Request;

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
            let buf: [u8; HANDSHAKE_LEN] = bytes[..HANDSHAKE_LEN].try_into().unwrap();
            return Self::Handshake(Handshake::from(buf));
        }

        let len = u32::from_be_bytes(bytes[..4].try_into().unwrap()) as usize;
        let end = len + 4;
        if end > n {
            return Self::Invalid(n);
        }

        let message_id = bytes[4];
        let payload = &bytes[5..end];

        match message_id {
            CHOKE_ID if len == 1 => Self::Choke,
            UNCHOKE_ID if len == 1 => Self::UnChoke,
            INTERESTED_ID if len == 1 => Self::Interested,
            NOT_INTERESTED_ID if len == 1 => Self::NotInterested,
            HAVE_ID if len == HAVE_LEN => {
                Self::Have(u32::from_be_bytes(payload.try_into().unwrap()))
            }
            BITFIELD_ID if len > 1 => Self::BitField(BitField::from(payload)),
            REQUEST_ID if len == REQUEST_LEN => {
                let buf: [u8; REQUEST_LEN - 1] = payload.try_into().unwrap();
                Self::Request(Request::from(buf))
            }
            CANCEL_ID if len == REQUEST_LEN => {
                let buf: [u8; REQUEST_LEN - 1] = payload.try_into().unwrap();
                Self::Cancel(Request::from(buf))
            }
            PIECE_ID if len > 8 => return Self::Piece(Piece::from(payload)),
            PORT_ID if len == PORT_LEN => {
                Self::Port(u16::from_be_bytes(payload.try_into().unwrap()))
            }
            _ => Message::Invalid(n),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::KeepAlive => KEEP_ALIVE_MESS.to_vec(),
            Self::Handshake(h) => h.bytes().to_vec(),
            Self::Choke => CHOKE_MESS.to_vec(),
            Self::UnChoke => UNCHOKE_MESS.to_vec(),
            Self::Interested => INTERESTED_MESS.to_vec(),
            Self::NotInterested => NOT_INTERESTED_MESS.to_vec(),
            Self::Have(i) => {
                let mut buf = [0u8; 4 + HAVE_LEN];

                buf[..5].copy_from_slice(&[0, 0, 0, HAVE_LEN as u8, HAVE_ID]);
                buf[5..].copy_from_slice(&i.to_be_bytes());
                buf.to_vec()
            }
            Self::BitField(b) => {
                let len = 1 + b.len();
                let mut buf = Vec::with_capacity(4 + len);

                buf.extend_from_slice(&(len as u32).to_be_bytes());
                buf.push(BITFIELD_ID);
                buf.extend_from_slice(b);
                buf
            }
            Self::Request(r) | Self::Cancel(r) => {
                let id = if let Self::Request(_) = self {
                    REQUEST_ID
                } else {
                    CANCEL_ID
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
                buf.push(PIECE_ID);
                buf.extend_from_slice(&p.index.to_be_bytes());
                buf.extend_from_slice(&p.begin.to_be_bytes());
                buf.extend_from_slice(&p.block);
                buf
            }
            Self::Port(p) => {
                let mut buf = [0u8; 4 + PORT_LEN];

                buf[..5].copy_from_slice(&[0, 0, 0, PORT_LEN as u8, PORT_ID]);
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
        }
    }
}

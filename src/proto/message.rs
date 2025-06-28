use crate::proto::bitfield::BitField;
use crate::proto::constants::*;
use crate::proto::handshake::Handshake;
use crate::proto::piece::Piece;
use crate::proto::request::Request;

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
            return Self::Handshake(Handshake::new(bytes[..HANDSHAKE_LEN].try_into().unwrap()));
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
            HAVE_ID if len == HAVE_LEN && payload.len() >= 4 => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&payload[..4]);
                Self::Have(u32::from_be_bytes(buf))
            }
            BITFIELD_ID => Self::BitField(BitField::from(payload)),
            REQUEST_ID if len == REQUEST_LEN => Self::Request(Request::from(payload)),
            CANCEL_ID if len == REQUEST_LEN => Self::Cancel(Request::from(payload)),
            PIECE_ID => return Self::Piece(Piece::from(payload)),
            PORT_ID if len == PORT_LEN && payload.len() >= 2 => {
                let mut buf = [0u8; 2];
                buf.copy_from_slice(&payload[..2]);
                Self::Port(u16::from_be_bytes(buf))
            }
            _ => Message::Invalid(n),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        fn serialize_request(r: &Request, id: u8) -> [u8; REQUEST_LEN] {
            let mut buf = [0u8; REQUEST_LEN];
            buf[..5].copy_from_slice(&[0, 0, 0, REQUEST_LEN as u8, id]);
            buf[5..HAVE_LEN].copy_from_slice(&r.index.to_be_bytes());
            buf[HAVE_LEN..13].copy_from_slice(&r.begin.to_be_bytes());
            buf[13..].copy_from_slice(&r.length.to_be_bytes());
            buf
        }

        match self {
            Self::KeepAlive => KEEP_ALIVE_MESS.into(),
            Self::Handshake(h) => h.bytes().into(),
            Self::Choke => CHOKE_MESS.into(),
            Self::UnChoke => UNCHOKE_MESS.into(),
            Self::Interested => INTERESTED_MESS.into(),
            Self::NotInterested => NOT_INTERESTED_MESS.into(),
            Self::Have(i) => {
                let mut buf = [0u8; HAVE_LEN];
                buf[..5].copy_from_slice(&[0, 0, 0, HAVE_LEN as u8, HAVE_ID]);
                buf[5..].copy_from_slice(&i.to_be_bytes());
                buf.into()
            }
            Self::BitField(b) => {
                let len = 1 + b.len();
                let mut buf = Vec::with_capacity(4 + len);

                buf.extend_from_slice(&(len as u32).to_be_bytes());
                buf.push(BITFIELD_ID);
                buf.extend_from_slice(b.as_slice());
                buf
            }
            Self::Request(r) => serialize_request(r, REQUEST_ID).into(),
            Self::Cancel(r) => serialize_request(r, CANCEL_ID).into(),
            Self::Piece(p) => {
                let len = HAVE_LEN + p.block.len();
                let mut buf = Vec::with_capacity(4 + len);

                buf.extend_from_slice(&(len as u32).to_be_bytes());
                buf.push(PIECE_ID);
                buf.extend_from_slice(&p.index.to_be_bytes());
                buf.extend_from_slice(&p.begin.to_be_bytes());
                buf.extend_from_slice(p.block.as_slice());
                buf
            }
            Self::Port(p) => {
                let mut buf = [0u8; PORT_LEN];
                buf[..5].copy_from_slice(&[0, 0, 0, PORT_LEN as u8, PORT_ID]);
                buf[5..].copy_from_slice(&p.to_be_bytes());
                buf.into()
            }
            _ => Vec::with_capacity(0),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Invalid(n) => *n,
            Self::KeepAlive => 4,
            Self::Handshake(_) => HANDSHAKE_LEN,
            Self::Choke | Self::UnChoke | Self::Interested | Self::NotInterested => 5,
            Self::Have(_) => HAVE_LEN,
            Self::BitField(b) => 5 + b.len(),
            Self::Request(_) | Self::Cancel(_) => REQUEST_LEN,
            Self::Piece(p) => 13 + p.block.len(),
            Self::Port(_) => PORT_LEN,
        }
    }
}

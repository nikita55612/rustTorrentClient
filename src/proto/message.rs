use super::bitfield::BitField;
use super::constants::*;
use super::handshake::Handshake;
use super::piece::Piece;
use super::request::Request;
use crate::proto::bep10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MSGage {
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

impl MSGage {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        match n {
            0 => return Self::Empty,
            x if x < 4 => return Self::Invalid(n),
            _ => (),
        }

        if bytes[..4] == KEEP_ALIVE_MSG {
            return Self::KeepAlive;
        }

        if n < 5 {
            return Self::Invalid(n);
        }

        if n >= HANDSHAKE_SIZE && bytes[..5] == HANDSHAKE_PREFIX {
            let buf: [u8; HANDSHAKE_SIZE] = bytes[..HANDSHAKE_SIZE].try_into().unwrap();
            return Self::Handshake(Handshake::new(buf));
        }

        let len = u32::from_be_bytes(bytes[..4].try_into().unwrap()) as usize;
        let end = len + 4;
        if end > n {
            return Self::Invalid(n);
        }

        let MSGage_id = bytes[4];
        let payload = &bytes[5..end];

        match MSGage_id {
            CHOKE_MSG_ID if len == 1 => Self::Choke,
            UNCHOKE_MSG_ID if len == 1 => Self::UnChoke,
            INTERESTED_MSG_ID if len == 1 => Self::Interested,
            NOT_INTERESTED_MSG_ID if len == 1 => Self::NotInterested,
            HAVE_MSG_ID if len == 1 + HAVE_PAYLOAD_LEN => {
                Self::Have(u32::from_be_bytes(payload.try_into().unwrap()))
            }
            BITFIELD_MSG_ID if len > 1 => Self::BitField(BitField(payload.into())),
            REQUEST_MSG_ID if len == 1 + REQUEST_PAYLOAD_LEN => {
                let buf: [u8; REQUEST_PAYLOAD_LEN] = payload.try_into().unwrap();
                Self::Request(Request::from_bytes(buf))
            }
            CANCEL_MSG_ID if len == 1 + REQUEST_PAYLOAD_LEN => {
                let buf: [u8; REQUEST_PAYLOAD_LEN] = payload.try_into().unwrap();
                Self::Cancel(Request::from_bytes(buf))
            }
            PIECE_MSG_ID if len > 8 => Self::Piece(Piece::from_bytes(payload)),
            PORT_MSG_ID if len == 1 + PORT_PAYLOAD_LEN => {
                Self::Port(u16::from_be_bytes(payload.try_into().unwrap()))
            }
            EXTENSION_MSG_ID if len > 6 => {
                Self::Extension(bep10::ExtensionMessage::from_bytes(&bytes[..end]))
            }
            _ => MSGage::Invalid(n),
        }
    }

    pub fn write_bytes(&self, buf: &mut [u8]) -> usize {
        match self {
            Self::KeepAlive => {
                buf[..4].copy_from_slice(&KEEP_ALIVE_MSG);
                4
            }
            Self::Handshake(h) => {
                buf[..HANDSHAKE_SIZE].copy_from_slice(h.as_slice());
                HANDSHAKE_SIZE
            }
            Self::Choke => {
                buf[..5].copy_from_slice(&CHOKE_MSG);
                5
            }
            Self::UnChoke => {
                buf[..5].copy_from_slice(&UNCHOKE_MSG);
                5
            }
            Self::Interested => {
                buf[..5].copy_from_slice(&INTERESTED_MSG);
                5
            }
            Self::NotInterested => {
                buf[..5].copy_from_slice(&NOT_INTERESTED_MSG);
                5
            }
            Self::Have(i) => {
                buf[..5].copy_from_slice(&HAVE_MSG_HEADER);
                buf[5..9].copy_from_slice(&i.to_be_bytes());
                9
            }
            Self::BitField(b) => {
                let len = b.len();
                buf[..4].copy_from_slice(&(1 + len as u32).to_be_bytes());
                buf[4] = BITFIELD_MSG_ID;
                buf[5..5 + len].copy_from_slice(&b);
                5 + len
            }
            Self::Request(r) | Self::Cancel(r) => {
                let id = if matches!(self, Self::Request(_)) {
                    REQUEST_MSG_ID
                } else {
                    CANCEL_MSG_ID
                };
                buf[..4].copy_from_slice(&REQUEST_MSG_HEADER[..4]);
                buf[4] = id;
                buf[5..].copy_from_slice(&r.to_bytes());
                17
            }
            Self::Piece(p) => {
                let len = p.len();
                buf[..4].copy_from_slice(&(1 + len as u32).to_be_bytes());
                buf[4] = PIECE_MSG_ID;
                buf[5..5 + len].copy_from_slice(&p.to_bytes());
                5 + len
            }
            Self::Port(p) => {
                buf[..5].copy_from_slice(&PORT_MSG_HEADER);
                buf[5..7].copy_from_slice(&p.to_be_bytes());
                7
            }
            Self::Extension(e) => {
                let bytes = e.to_bytes();
                let len = bytes.len();
                buf[..len].copy_from_slice(&bytes);
                len
            }
            _ => {
                buf[..4].copy_from_slice(&KEEP_ALIVE_MSG);
                4
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::KeepAlive => KEEP_ALIVE_MSG.to_vec(),
            Self::Handshake(h) => h.to_vec(),
            Self::Choke => CHOKE_MSG.to_vec(),
            Self::UnChoke => UNCHOKE_MSG.to_vec(),
            Self::Interested => INTERESTED_MSG.to_vec(),
            Self::NotInterested => NOT_INTERESTED_MSG.to_vec(),
            Self::Have(i) => {
                let mut buf = vec![0u8; 4 + 1 + HAVE_PAYLOAD_LEN];
                buf[..5].copy_from_slice(&HAVE_MSG_HEADER);
                buf[5..].copy_from_slice(&i.to_be_bytes());
                buf
            }
            Self::BitField(b) => {
                let len = b.len();
                let mut buf = vec![0u8; 4 + 1 + len];
                buf[..4].copy_from_slice(&(1 + len as u32).to_be_bytes());
                buf[4] = BITFIELD_MSG_ID;
                buf[5..].copy_from_slice(&b);
                buf
            }
            Self::Request(r) | Self::Cancel(r) => {
                let id = if let Self::Request(_) = self {
                    REQUEST_MSG_ID
                } else {
                    CANCEL_MSG_ID
                };
                let mut buf = vec![0u8; 4 + 1 + REQUEST_PAYLOAD_LEN];
                buf[..4].copy_from_slice(&REQUEST_MSG_HEADER[..4]);
                buf[4] = id;
                buf[5..].copy_from_slice(&r.to_bytes());
                buf
            }
            Self::Piece(p) => {
                let len = p.len();
                let mut buf = vec![0u8; 4 + 1 + len];
                buf[..4].copy_from_slice(&(1 + len as u32).to_be_bytes());
                buf[4] = PIECE_MSG_ID;
                buf[5..].copy_from_slice(&p.to_bytes());
                buf
            }
            Self::Port(p) => {
                let mut buf = vec![0u8; 4 + 1 + PORT_PAYLOAD_LEN];
                buf[..5].copy_from_slice(&PORT_MSG_HEADER);
                buf[5..].copy_from_slice(&p.to_be_bytes());
                buf
            }
            _ => Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Invalid(n) => *n,
            Self::KeepAlive => 4,
            Self::Handshake(_) => HANDSHAKE_SIZE,
            Self::Choke | Self::UnChoke | Self::Interested | Self::NotInterested => 4 + 1,
            Self::Have(_) => 4 + 1 + HAVE_PAYLOAD_LEN,
            Self::BitField(b) => 4 + 1 + b.len(),
            Self::Request(_) | Self::Cancel(_) => 4 + 1 + REQUEST_PAYLOAD_LEN,
            Self::Piece(p) => 4 + 1 + p.len(),
            Self::Port(_) => 4 + 1 + PORT_PAYLOAD_LEN,
            Self::Extension(e) => e.len(),
        }
    }
}

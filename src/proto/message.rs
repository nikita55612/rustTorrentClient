use crate::proto::bitfield::BitField;
use crate::proto::handshake::Handshake;

pub const CHOKE_ID: u8 = 0;
pub const UNCHOKE_ID: u8 = 1;
pub const INTERESTED_ID: u8 = 2;
pub const NOT_INTERESTED_ID: u8 = 3;
pub const HAVE_ID: u8 = 4;
pub const BITFIELD_ID: u8 = 5;
pub const REQUEST_ID: u8 = 6;
pub const PIECE_ID: u8 = 7;
pub const CANCEL_ID: u8 = 8;
pub const PORT_ID: u8 = 9;

pub const HANDSHAKE_PREFIX: [u8; 4] = [19, 66, 105, 116];
pub const KEEP_ALIVE_MESS: [u8; 4] = [0, 0, 0, 0];

pub enum Message {
    Empty,
    Invalid,
    KeepAlive,
    Choke,
    UnChoke,
    Interested,
    NotInterested,
    Have(u32),
    BitField(BitField),
    // Request(Request),
    // Cancel(PeerRequest),
    // Piece(Piece),
    Handshake(Handshake),
    Port(u16),
}

#[allow(dead_code)]
impl Message {
    fn from_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        if n == 0 {
            return Self::Empty;
        }
        if bytes == KEEP_ALIVE_MESS {
            return Self::KeepAlive;
        }
        if n < 5 {
            return Self::Invalid;
        }

        if bytes[..4] == HANDSHAKE_PREFIX && n >= 68 {
            return Self::Handshake(Handshake::new(bytes[..68].try_into().unwrap()));
        }

        let message_id = bytes[4];
        match message_id {
            CHOKE_ID => return Self::Choke,
            UNCHOKE_ID => return Self::UnChoke,
            INTERESTED_ID => return Self::Interested,
            NOT_INTERESTED_ID => return Self::NotInterested,
            _ => (),
        }

        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&bytes[..4]);
        let len = u32::from_be_bytes(len_bytes) as usize;

        let payload_len = len - 1;
        let mut payload = Vec::with_capacity(payload_len);
        if payload_len > 0 {
            payload.extend_from_slice(&bytes[5..]);
        }

        match message_id {
            HAVE_ID => {
                if payload_len >= 4 {
                    let mut have_payload = [0u8; 4];
                    have_payload.copy_from_slice(&payload[..4]);
                    return Self::Have(u32::from_be_bytes(have_payload));
                }
                return Message::Invalid;
            }
            BITFIELD_ID => return Self::BitField(BitField::new()),
            // REQUEST_ID => return Self::Request(()),
            // CANCEL_ID => return Self::Cancel(()),
            // PIECE_ID => return Self::Piece(()),
            _ => return Message::Invalid,
        }
    }
}

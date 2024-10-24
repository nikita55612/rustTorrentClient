

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum MessageID {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
    Unknown = 255
}

impl From<u8> for MessageID {
    fn from(id: u8) -> Self {
        if id <= 8 {
            unsafe { std::mem::transmute(id) }
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub id: MessageID,
    pub payload: Vec<u8>
}

impl Message {
    pub fn new(id: MessageID, payload: Vec<u8>) -> Self {
        Self {
            id,
            payload
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let id = match bytes.get(0) {
            Some(id) => MessageID::from(*id),
            None => MessageID::Unknown
        };
        Self {
            id,
            payload: bytes[1..].to_vec()
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let length = (self.payload.len() + 1) as u32;
        let mut buffer = Vec::with_capacity((length + 4) as usize);
        buffer.extend_from_slice(&length.to_be_bytes());
        buffer.push(self.id as u8);
        buffer.extend_from_slice(&self.payload);
        buffer
    }
}
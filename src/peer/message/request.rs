use crate::peer::message::constants::REQUEST_LEN;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Request {
    pub index: u32,
    pub begin: u32,
    pub length: u32,
}

impl From<[u8; REQUEST_LEN - 1]> for Request {
    fn from(buf: [u8; REQUEST_LEN - 1]) -> Self {
        Self {
            index: u32::from_be_bytes(buf[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(buf[4..8].try_into().unwrap()),
            length: u32::from_be_bytes(buf[8..12].try_into().unwrap()),
        }
    }
}

impl Request {
    pub fn new(index: u32, begin: u32, length: u32) -> Self {
        Self {
            index: index,
            begin: begin,
            length: length,
        }
    }
}

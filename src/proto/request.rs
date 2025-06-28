#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Request {
    pub index: u32,
    pub begin: u32,
    pub length: u32,
}

impl From<Vec<u8>> for Request {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from(bytes.as_slice())
    }
}

impl From<&[u8]> for Request {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() < 12 {
            return Request::default();
        }
        Self {
            index: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            length: u32::from_be_bytes(bytes[8..12].try_into().unwrap()),
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

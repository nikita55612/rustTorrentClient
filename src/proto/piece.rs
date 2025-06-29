#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Piece {
    pub index: u32,
    pub begin: u32,
    pub block: Vec<u8>,
}

impl From<&[u8]> for Piece {
    fn from(bytes: &[u8]) -> Self {
        if bytes.len() < 8 {
            return Self::default();
        }
        Self {
            index: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            block: bytes[8..].to_vec(),
        }
    }
}

impl From<Vec<u8>> for Piece {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from(bytes.as_slice())
    }
}

impl Piece {
    pub fn new(index: u32, begin: u32, block: Vec<u8>) -> Self {
        Self {
            index,
            begin,
            block,
        }
    }
}

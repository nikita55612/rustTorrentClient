#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Piece {
    pub index: u32,
    pub begin: u32,
    pub block: Vec<u8>,
}

impl Piece {
    pub fn new(index: u32, begin: u32, block: Vec<u8>) -> Self {
        Self {
            index,
            begin,
            block,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let n = bytes.len();
        if n < 8 {
            return Self::default();
        }
        Self {
            index: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            begin: u32::from_be_bytes(bytes[4..8].try_into().unwrap()),
            block: bytes[8..].to_vec(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; self.len()];

        buf[0..4].copy_from_slice(&self.index.to_be_bytes());
        buf[4..8].copy_from_slice(&self.begin.to_be_bytes());
        buf[8..].copy_from_slice(&self.block);

        buf
    }

    #[inline]
    pub fn len(&self) -> usize {
        8 + self.block.len()
    }
}

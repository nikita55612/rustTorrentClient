use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitField(pub Vec<u8>);

impl Deref for BitField {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BitField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BitField {
    pub fn new(num_pieces: usize) -> Self {
        let num_bytes = (num_pieces + 7) / 8;
        Self(vec![0; num_bytes])
    }

    pub fn set(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_offset = 7 - (index % 8);
        self.0[byte_index] |= 1 << bit_offset;
    }

    pub fn has(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_offset = 7 - (index % 8);
        (self.0[byte_index] & (1 << bit_offset)) != 0
    }
}

use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitField(Vec<u8>);

impl Deref for BitField {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&[u8]> for BitField {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

impl BitField {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn has_piece(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = 7 - (index % 8);

        self.0
            .get(byte_index)
            .map(|byte| (byte >> bit_index) & 1 != 0)
            .unwrap_or(false)
    }

    pub fn set_piece(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = 7 - (index % 8);

        if byte_index >= self.0.len() {
            self.0.resize(byte_index + 1, 0);
        }
        self.0[byte_index] |= 1 << bit_index;
    }
}

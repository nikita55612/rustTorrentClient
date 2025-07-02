use std::ops::Deref;

pub struct Piece(Vec<u8>);

impl Deref for Piece {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Piece {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn is_done(&self) -> bool {
        self.len() == self.capacity()
    }
}

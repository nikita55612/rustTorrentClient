use super::ExtendedHandshake;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionMessage {
    Empty,
    Handshake(ExtendedHandshake),
}

impl ExtensionMessage {
    pub fn from_bytes(_bytes: &[u8]) -> Self {
        Self::Empty
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Handshake(_) => 0,
        }
    }
}

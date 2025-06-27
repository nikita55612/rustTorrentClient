use sha1::{Digest, Sha1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 20]);

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex())
    }
}

impl Hash {
    pub fn new(s: [u8; 20]) -> Self {
        Self(s)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = [0u8; 20];
        hash.copy_from_slice(&Sha1::digest(bytes));
        Self(hash)
    }

    pub fn bytes(&self) -> &[u8; 20] {
        &self.0
    }

    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn percent_encoding(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789ABCDEF";
        let mut encoded = String::with_capacity(20 * 3);
        for &b in &self.0 {
            encoded.push('%');
            encoded.push(HEX[(b >> 4) as usize] as char);
            encoded.push(HEX[(b & 0x0F) as usize] as char);
        }
        encoded
    }
}

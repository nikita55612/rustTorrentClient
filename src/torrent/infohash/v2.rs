use super::InfoHashT;
use crate::error::{Error, Result};
use crate::torrent::infohash::INFO_HASH_V1_SIZE;
use crate::util::urlencode;
use sha2::{Digest, Sha256};
use std::ops::{Deref, DerefMut};

pub const INFO_HASH_V2_SIZE: usize = 32;
pub const INFO_HASH_V2_HEX_SIZE: usize = 64;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InfoHashV2([u8; INFO_HASH_V2_SIZE]);

impl Deref for InfoHashV2 {
    type Target = [u8; INFO_HASH_V2_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfoHashV2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InfoHashT for InfoHashV2 {
    fn hex(&self) -> String {
        self.hex()
    }

    fn urlencode(&self) -> String {
        self.urlencode()
    }

    fn as_bytes(&self) -> &[u8] {
        self.as_slice()
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }

    fn truncate(&self) -> &[u8; INFO_HASH_V1_SIZE] {
        self[..INFO_HASH_V1_SIZE].try_into().unwrap()
    }

    fn len(&self) -> usize {
        INFO_HASH_V2_SIZE
    }
}

impl InfoHashV2 {
    pub fn new(buf: [u8; INFO_HASH_V2_SIZE]) -> Self {
        Self(buf)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = [0u8; INFO_HASH_V2_SIZE];
        hash.copy_from_slice(&Sha256::digest(bytes));

        Self(hash)
    }

    pub fn from_hex(data: &[u8; INFO_HASH_V2_HEX_SIZE]) -> Result<Self> {
        let hash: [u8; INFO_HASH_V2_SIZE] = hex::decode(data)?.try_into().unwrap();
        Ok(Self(hash))
    }

    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn urlencode(&self) -> String {
        urlencode(self.truncate())
    }
}

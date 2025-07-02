use crate::error::{Error, Result};
use crate::types::infohash::InfoHash;
use crate::util::urlencode;
use sha1::{Digest, Sha1};
use std::ops::{Deref, DerefMut};

const SIZE: usize = 20;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InfoHashV1([u8; SIZE]);

impl Deref for InfoHashV1 {
    type Target = [u8; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfoHashV1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InfoHash for InfoHashV1 {
    fn from_bytes(bytes: &[u8]) -> Self {
        InfoHashV1::from_bytes(bytes)
    }

    fn from_hex(s: &str) -> Result<Self> {
        InfoHashV1::from_hex(s)
    }

    fn hex(&self) -> String {
        self.hex()
    }

    fn urlencode(&self) -> String {
        urlencode(&self.0)
    }

    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        self.as_mut_bytes()
    }

    fn truncated_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

    fn len(&self) -> usize {
        SIZE
    }
}

impl InfoHashV1 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = [0u8; SIZE];
        hash.copy_from_slice(&Sha1::digest(bytes));

        Self(hash)
    }

    pub fn from_hex(s: &str) -> Result<Self> {
        let hash: [u8; SIZE] = hex::decode(s)?
            .try_into()
            .map_err(|_| Error::Custom(format!("From Vec<u8> to [u8; {SIZE}] error.")))?;
        Ok(Self(hash))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn as_mut_bytes(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn urlencode(&self) -> String {
        urlencode(&self.0)
    }
}

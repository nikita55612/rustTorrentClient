use super::InfoHashT;
use crate::error::{Error, Result};
use crate::util::urlencode;
use sha2::{Digest, Sha256};
use std::ops::{Deref, DerefMut};

const SIZE: usize = 32;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InfoHashV2([u8; SIZE]);

impl Deref for InfoHashV2 {
    type Target = [u8; SIZE];

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

    fn truncated_bytes(&self) -> &[u8] {
        &self.0[..20]
    }

    fn len(&self) -> usize {
        SIZE
    }
}

impl InfoHashV2 {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = [0u8; SIZE];
        hash.copy_from_slice(&Sha256::digest(bytes));

        Self(hash)
    }

    pub fn from_hex(s: &str) -> Result<Self> {
        let hash: [u8; SIZE] = hex::decode(s)?
            .try_into()
            .map_err(|_| Error::Custom(format!("From Vec<u8> to [u8; {SIZE}] error.")))?;
        Ok(Self(hash))
    }

    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn urlencode(&self) -> String {
        let truncated_bytes = self.truncated_bytes();
        urlencode(truncated_bytes)
    }
}

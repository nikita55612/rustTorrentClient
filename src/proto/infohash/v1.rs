use super::InfoHashT;
use crate::error::{Error, Result};
use crate::proto::constants::{INFO_HASH_V1_HEX_SIZE, INFO_HASH_V1_SIZE};
use crate::util::urlencode;
use sha1::{Digest, Sha1};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InfoHashV1([u8; INFO_HASH_V1_SIZE]);

impl Deref for InfoHashV1 {
    type Target = [u8; INFO_HASH_V1_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InfoHashV1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl InfoHashT for InfoHashV1 {
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
        &self
    }

    fn len(&self) -> usize {
        INFO_HASH_V1_SIZE
    }
}

impl InfoHashV1 {
    pub fn new(buf: [u8; INFO_HASH_V1_SIZE]) -> Self {
        Self(buf)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hash = [0u8; INFO_HASH_V1_SIZE];
        hash.copy_from_slice(&Sha1::digest(bytes));

        Self(hash)
    }

    pub fn from_hex(data: &[u8; INFO_HASH_V1_HEX_SIZE]) -> Result<Self> {
        let hash: [u8; INFO_HASH_V1_SIZE] = hex::decode(data)?.try_into().unwrap();
        Ok(Self(hash))
    }

    pub fn from_base32(s: &str) -> Result<Self> {
        let hash: [u8; INFO_HASH_V1_SIZE] =
            base32::decode(base32::Alphabet::Rfc4648 { padding: false }, s)
                .ok_or(())
                .map_err(|_| Error::Custom("Decode base32 rfc4648 error".into()))?
                .try_into()
                .map_err(|_| {
                    Error::Custom(format!("From Vec<u8> to [u8; {INFO_HASH_V1_SIZE}] error"))
                })?;
        Ok(Self(hash))
    }

    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn urlencode(&self) -> String {
        urlencode(&self.0)
    }
}

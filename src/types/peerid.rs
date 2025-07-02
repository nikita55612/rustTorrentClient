use rand::Rng;
use std::ops::{Deref, DerefMut};

use crate::util::urlencode;

const SIZE: usize = 20;
const DEFAULT_PEER_ID_PREFIX: &str = "-UT2210-";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PeerId([u8; SIZE]);

impl Deref for PeerId {
    type Target = [u8; SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PeerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[u8; SIZE]> for PeerId {
    fn from(buf: [u8; SIZE]) -> Self {
        Self(buf)
    }
}

impl PeerId {
    pub fn gen_new() -> Self {
        Self::gen_with_prefix(DEFAULT_PEER_ID_PREFIX.as_bytes().try_into().unwrap())
    }

    pub fn gen_with_prefix(p: &[u8; 8]) -> Self {
        let mut buf = [0u8; SIZE];
        buf[..8].copy_from_slice(p);

        let rng = rand::rng();
        let iterator = rng
            .sample_iter(rand::distr::Alphanumeric)
            .take(12)
            .enumerate();

        for (i, byte) in iterator {
            buf[8 + i] = byte;
        }

        Self(buf)
    }

    pub fn urlencode(&self) -> String {
        urlencode(&self.0)
    }
}

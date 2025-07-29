use super::constants::DEFAULT_PEER_FINGERPRINT;
use crate::{proto::constants::PEER_ID_SIZE, util::urlencode};
use rand::Rng;
use std::ops::{Deref, DerefMut};

type Fingerprint = [u8; 8];

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PeerId([u8; PEER_ID_SIZE]);

impl Deref for PeerId {
    type Target = [u8; PEER_ID_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PeerId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PeerId {
    pub fn new(buf: [u8; PEER_ID_SIZE]) -> Self {
        Self(buf)
    }

    pub fn gen_new() -> Self {
        Self::gen_with_fingerprint(DEFAULT_PEER_FINGERPRINT)
    }

    pub fn gen_with_fingerprint(fingerprint: &Fingerprint) -> Self {
        let mut buf = [0u8; PEER_ID_SIZE];
        buf[..8].copy_from_slice(fingerprint);

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

    pub fn extract_header(&self) -> [u8; 8] {
        self.0[..8].try_into().unwrap()
    }
}

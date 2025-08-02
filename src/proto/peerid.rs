use super::constants::DEFAULT_PEER_FINGERPRINT;
use crate::{
    proto::constants::{PEER_ID_FINGERPRINT_SIZE, PEER_ID_SIZE},
    util::urlencode,
};
use rand::Rng;
use std::ops::{Deref, DerefMut};

type Fingerprint = [u8; PEER_ID_FINGERPRINT_SIZE];

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
        buf[..PEER_ID_FINGERPRINT_SIZE].copy_from_slice(fingerprint);

        let rng = rand::rng();
        let iterator = rng
            .sample_iter(rand::distr::Alphanumeric)
            .take(PEER_ID_SIZE - PEER_ID_FINGERPRINT_SIZE)
            .enumerate();

        for (i, byte) in iterator {
            buf[PEER_ID_FINGERPRINT_SIZE + i] = byte;
        }

        Self(buf)
    }

    pub fn urlencode(&self) -> String {
        urlencode(&self.0)
    }

    pub fn extract_fingerprint(&self) -> Fingerprint {
        *self.select_fingerprint()
    }

    #[inline]
    pub fn select_fingerprint(&self) -> &Fingerprint {
        self[..PEER_ID_FINGERPRINT_SIZE].try_into().unwrap()
    }
}

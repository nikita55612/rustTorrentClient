use super::constants::{HANDSHAKE_PSTR, HANDSHAKE_SIZE};
use super::PeerId;
use crate::proto::constants::PEER_ID_SIZE;
use crate::torrent::infohash::{InfoHash, InfoHashV1, INFO_HASH_V1_SIZE};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake([u8; HANDSHAKE_SIZE]);

impl Deref for Handshake {
    type Target = [u8; HANDSHAKE_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Handshake {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Handshake {
    pub fn new(buf: [u8; HANDSHAKE_SIZE]) -> Self {
        Self(buf)
    }

    pub fn from_args(info_hash: &InfoHash, peer_id: &PeerId) -> Self {
        let mut buf = [0u8; HANDSHAKE_SIZE];
        let pstr_len = HANDSHAKE_PSTR.len();
        let mut curr = 0;

        buf[curr] = pstr_len as u8;
        curr += 1;

        buf[curr..curr + pstr_len].copy_from_slice(&HANDSHAKE_PSTR);
        curr += pstr_len;

        buf[curr..curr + 8].copy_from_slice(&[0; 8]);
        curr += 8;

        buf[curr..curr + 20].copy_from_slice(info_hash.inner().as_bytes());
        curr += 20;

        buf[curr..curr + 20].copy_from_slice(peer_id.as_slice());

        Self(buf)
    }

    pub fn extract_info_hash(&self) -> InfoHash {
        InfoHash::V1(InfoHashV1::new(*self.select_info_hash()))
    }

    #[inline]
    pub fn select_info_hash(&self) -> &[u8; INFO_HASH_V1_SIZE] {
        self[HANDSHAKE_SIZE - (PEER_ID_SIZE + INFO_HASH_V1_SIZE)..HANDSHAKE_SIZE - PEER_ID_SIZE]
            .try_into()
            .unwrap()
    }

    pub fn extract_peer_id(&self) -> PeerId {
        PeerId::new(*self.select_peer_id())
    }

    #[inline]
    pub fn select_peer_id(&self) -> &[u8; PEER_ID_SIZE] {
        self[HANDSHAKE_SIZE - PEER_ID_SIZE..].try_into().unwrap()
    }
}

use super::constants::{HANDSHAKE_LEN, HANDSHAKE_PSTR};
use super::PeerId;
use crate::torrent::infohash::{InfoHash, InfoHashV1};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake(pub [u8; HANDSHAKE_LEN]);

impl Deref for Handshake {
    type Target = [u8; HANDSHAKE_LEN];

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
    pub fn new(info_hash: &InfoHash, peer_id: &PeerId) -> Self {
        let mut buf = [0u8; HANDSHAKE_LEN];
        let pstr_len = HANDSHAKE_PSTR.len();
        let mut curr = 0;

        buf[curr] = pstr_len as u8;
        curr += 1;

        buf[curr..curr + pstr_len].copy_from_slice(HANDSHAKE_PSTR);
        curr += pstr_len;

        buf[curr..curr + 8].copy_from_slice(&[0; 8]);
        curr += 8;

        buf[curr..curr + 20].copy_from_slice(info_hash.inner().as_bytes());
        curr += 20;

        buf[curr..curr + 20].copy_from_slice(peer_id.as_slice());

        Self(buf)
    }

    pub fn extract_info_hash(&self) -> InfoHash {
        let mut info_hash = InfoHashV1::default();
        info_hash.copy_from_slice(&self.0[HANDSHAKE_LEN - 40..HANDSHAKE_LEN - 20]);
        InfoHash::V1(info_hash)
    }

    pub fn extract_peer_id(&self) -> PeerId {
        let mut peer_id = PeerId::default();
        peer_id.copy_from_slice(&self.0[HANDSHAKE_LEN - 20..]);
        peer_id
    }
}

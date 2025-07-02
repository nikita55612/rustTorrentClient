use crate::peer::message::constants::{HANDSHAKE_LEN, HANDSHAKE_PSTR};
use crate::types::infohash::{InfoHash, InfoHashV1};
use crate::types::PeerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake([u8; HANDSHAKE_LEN]);

impl From<[u8; HANDSHAKE_LEN]> for Handshake {
    fn from(buf: [u8; HANDSHAKE_LEN]) -> Self {
        Self(buf)
    }
}

impl Handshake {
    pub fn from_parts(info_hash: impl InfoHash, peer_id: &PeerId) -> Self {
        let mut buf = [0u8; HANDSHAKE_LEN];
        let pstr_len = HANDSHAKE_PSTR.len();
        let mut curr = 0;

        buf[curr] = pstr_len as u8;
        curr += 1;

        buf[curr..curr + pstr_len].copy_from_slice(HANDSHAKE_PSTR.as_bytes());
        curr += pstr_len;

        buf[curr..curr + 8].copy_from_slice(&[0; 8]);
        curr += 8;

        buf[curr..curr + 20].copy_from_slice(info_hash.as_bytes());
        curr += 20;

        buf[curr..curr + 20].copy_from_slice(peer_id.as_slice());

        Self(buf)
    }

    pub fn bytes(&self) -> &[u8; HANDSHAKE_LEN] {
        &self.0
    }

    pub fn extract_info_hash(&self) -> impl InfoHash {
        let mut info_hash = InfoHashV1::default();
        info_hash.copy_from_slice(&self.0[HANDSHAKE_LEN - 40..HANDSHAKE_LEN - 20]);
        info_hash
    }

    pub fn extract_peer_id(&self) -> PeerId {
        let mut peer_id = PeerId::default();
        peer_id.copy_from_slice(&self.0[HANDSHAKE_LEN - 20..]);
        peer_id
    }
}

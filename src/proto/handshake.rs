use crate::bencode::InfoHash;
use crate::peer::PeerId;

pub const PSTR: &'static str = "BitTorrent protocol";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handshake([u8; 68]);

impl Handshake {
    pub fn new(s: [u8; 68]) -> Self {
        Self(s)
    }

    pub fn from_parts(info_hash: &InfoHash, peer_id: &PeerId) -> Self {
        let mut buf = [0u8; 68];
        let pstr_len = PSTR.len();
        let mut curr: usize = 0;

        buf[curr] = pstr_len as u8;
        curr += 1;
        buf[curr..curr + pstr_len].copy_from_slice(PSTR.as_bytes());
        curr += pstr_len;
        buf[curr..curr + 8].copy_from_slice(&[0; 8]);
        curr += 8;
        buf[curr..curr + 20].copy_from_slice(info_hash.bytes());
        curr += 20;
        buf[curr..curr + 20].copy_from_slice(peer_id.bytes());

        Self(buf)
    }

    pub fn bytes(&self) -> &[u8; 68] {
        &self.0
    }

    pub fn extract_info_hash(&self) -> InfoHash {
        let mut s = [0u8; 20];
        s.copy_from_slice(&self.0[68 - 20 * 2..68 - 20]);
        InfoHash::new(s)
    }

    pub fn extract_peer_id(&self) -> PeerId {
        let mut s = [0u8; 20];
        s.copy_from_slice(&self.0[68 - 20..68]);
        PeerId::new(s)
    }
}

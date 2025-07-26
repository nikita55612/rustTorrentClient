use super::{InfoHashV1, InfoHashV2};

pub trait InfoHashT {
    fn hex(&self) -> String;

    fn urlencode(&self) -> String;

    fn as_bytes(&self) -> &[u8];

    fn as_mut_bytes(&mut self) -> &mut [u8];

    fn truncated_bytes(&self) -> &[u8];

    fn len(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfoHash {
    V1(InfoHashV1),
    V2(InfoHashV2),
}

impl InfoHash {
    pub fn inner(&self) -> &dyn InfoHashT {
        match self {
            Self::V1(info_hash) => info_hash,
            Self::V2(info_hash) => info_hash,
        }
    }

    pub fn inner_mut(&mut self) -> &mut dyn InfoHashT {
        match self {
            Self::V1(info_hash) => info_hash,
            Self::V2(info_hash) => info_hash,
        }
    }
}

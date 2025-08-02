use super::{InfoHashV1, InfoHashV2};
use crate::proto::constants::INFO_HASH_V1_SIZE;

pub trait InfoHashT {
    fn hex(&self) -> String;

    fn urlencode(&self) -> String;

    fn as_bytes(&self) -> &[u8];

    fn as_mut_bytes(&mut self) -> &mut [u8];

    fn truncate(&self) -> &[u8; INFO_HASH_V1_SIZE];

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

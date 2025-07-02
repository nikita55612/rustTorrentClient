use crate::error::Result;

pub trait InfoHash {
    fn from_bytes(bytes: &[u8]) -> Self;

    fn from_hex(s: &str) -> Result<Self>
    where
        Self: Sized;

    fn hex(&self) -> String;

    fn urlencode(&self) -> String;

    fn as_bytes(&self) -> &[u8];

    fn as_mut_bytes(&mut self) -> &mut [u8];

    fn truncated_bytes(&self) -> &[u8];

    fn len(&self) -> usize;
}

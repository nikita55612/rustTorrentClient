/// https://bittorrent.org/beps/bep_0015.html#connect
use super::constants::{BEP15_CONNECT_LEN, BEP15_MAGIC_CONSTANT};
use crate::error::{Error, Result};

pub struct ConnectRequest {
    pub transaction_id: u32,
}

impl ConnectRequest {
    pub fn new() -> Self {
        let transaction_id = rand::random();
        Self { transaction_id }
    }

    pub fn to_bytes(&self) -> [u8; BEP15_CONNECT_LEN] {
        let mut buf = [0u8; BEP15_CONNECT_LEN];

        buf[..8].copy_from_slice(&BEP15_MAGIC_CONSTANT);
        buf[8..12].copy_from_slice(&0u32.to_be_bytes());
        buf[12..16].copy_from_slice(&self.transaction_id.to_be_bytes());

        buf
    }
}

#[derive(Debug)]
pub struct ConnectResponse {
    pub transaction_id: u32,
    pub connection_id: u64,
}

impl ConnectResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < BEP15_CONNECT_LEN {
            return Result::Err(Error::InvalidBep15ConnectResponse(
                "Response too short".into(),
            ));
        }

        let action = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let transaction_id = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let connection_id = u64::from_be_bytes(bytes[8..BEP15_CONNECT_LEN].try_into().unwrap());

        if action != 0 {
            return Result::Err(Error::InvalidBep15ConnectResponse(format!(
                "Unexpected action code: {}",
                action
            )));
        }

        Ok(Self {
            transaction_id,
            connection_id,
        })
    }
}

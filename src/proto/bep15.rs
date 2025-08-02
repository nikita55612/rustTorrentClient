/// <https://bittorrent.org/beps/bep_0015.html#connect>
use super::constants::{BEP15_CONNECT_LEN, BEP15_MAGIC_CONSTANT};
use crate::{
    error::{Error, Result},
    proto::constants::BEP15_MIN_MSG_LEN,
};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::atomic::{AtomicI32, Ordering},
};

pub type Bep15TransactionID = i32;

static GLOBAL_TID_COUNTER: AtomicI32 = AtomicI32::new(42);

pub fn fetch_add_bep15_transaction_id() -> Bep15TransactionID {
    GLOBAL_TID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[derive(Clone, Debug)]
pub enum Bep15Response {
    Connect(Bep15ConnectResponse),
    Announce(Bep15AnnounceResponse),
    // Scrape,
    Error {
        transaction_id: Bep15TransactionID,
        message: String,
    },
}

impl Bep15Response {
    pub fn transaction_id(&self) -> Bep15TransactionID {
        match &self {
            Self::Connect(conn) => conn.transaction_id,
            Self::Announce(ann) => ann.transaction_id,
            Self::Error { transaction_id, .. } => *transaction_id,
        }
    }
}

impl Bep15Response {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < BEP15_MIN_MSG_LEN {
            return Result::Err(Error::InvalidBep15Response("Response too short".into()));
        }
        let action = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        Ok(match action {
            0 => Self::Connect(Bep15ConnectResponse::from_bytes(bytes)?),
            1 => Self::Announce(Bep15AnnounceResponse::from_bytes(bytes)?),
            // 2 => Scrape,
            3 => {
                let transaction_id =
                    Bep15TransactionID::from_be_bytes(bytes[4..8].try_into().unwrap());
                let message = String::from_utf8(bytes[8..].to_vec()).unwrap_or_default();
                Self::Error {
                    transaction_id,
                    message,
                }
            }
            _ => {
                return Result::Err(Error::InvalidBep15Response(format!(
                    "Invalid action response: {}",
                    action
                )))
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct Bep15ConnectRequest {
    pub transaction_id: Bep15TransactionID,
}

impl Bep15ConnectRequest {
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

#[derive(Clone, Debug)]
pub struct Bep15ConnectResponse {
    pub transaction_id: Bep15TransactionID,
    pub connection_id: i64,
}

impl Bep15ConnectResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < BEP15_CONNECT_LEN {
            return Result::Err(Error::InvalidBep15Response(
                "Connect response too short".into(),
            ));
        }

        let action = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let transaction_id = Bep15TransactionID::from_be_bytes(bytes[4..8].try_into().unwrap());
        let connection_id = i64::from_be_bytes(bytes[8..BEP15_CONNECT_LEN].try_into().unwrap());

        if action != 0 {
            return Result::Err(Error::InvalidBep15Response(format!(
                "Invalid action in connect response: {}",
                action
            )));
        }

        Ok(Self {
            transaction_id,
            connection_id,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Bep15AnnounceResponse {
    pub transaction_id: Bep15TransactionID,
    pub interval: u32,
    pub leechers: u32,
    pub seeders: u32,
    pub peers: Vec<SocketAddr>,
}

impl Bep15AnnounceResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 20 {
            return Result::Err(Error::InvalidBep15Response(
                "Announce response too short".into(),
            ));
        }

        let action = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        if action != 1 {
            return Result::Err(Error::InvalidBep15Response(
                "Invalid action in announce response".into(),
            ));
        }

        let transaction_id = Bep15TransactionID::from_be_bytes(bytes[4..8].try_into().unwrap());
        let interval = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let leechers = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        let seeders = u32::from_be_bytes(bytes[16..20].try_into().unwrap());

        let mut peers = Vec::new();
        let mut offset = 20;

        while offset + 6 <= bytes.len() {
            let ip = IpAddr::V4(Ipv4Addr::new(
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ));
            let port = u16::from_be_bytes(bytes[offset + 4..offset + 6].try_into().unwrap());
            peers.push(SocketAddr::new(ip, port));
            offset += 6;
        }

        Ok(Self {
            transaction_id,
            interval,
            leechers,
            seeders,
            peers,
        })
    }
}

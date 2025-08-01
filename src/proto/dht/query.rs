use std::collections::BTreeMap;

use crate::proto::constants::{
    DHT_ANNOUNCE_PEER_QUERY_STR, DHT_FIND_NODE_QUERY_STR, DHT_GET_PEERS_QUERY_STR,
    DHT_PING_QUERY_STR,
};

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Default, Clone)]
pub enum QueryArgs {
    Ping {
        id: Vec<u8>,
    },
    FindNode {
        id: Vec<u8>,
        target: Vec<u8>,
    },
    GetPeers {
        id: Vec<u8>,
        info_hash: Vec<u8>,
    },
    AnnouncePeer {
        id: Vec<u8>,
        info_hash: Vec<u8>,
        port: i64,
        token: Vec<u8>,
        implied_port: i64,
    },
    Other(BTreeMap<String, BencodeValue>),

    #[default]
    None,
}

impl QueryArgs {
    pub fn parse(q: &str, a_map: Option<BTreeMap<String, BencodeValue>>) -> Self {
        let mut map = if let Some(d) = a_map {
            d
        } else {
            return Self::None;
        };
        match q {
            DHT_PING_QUERY_STR => Self::Ping {
                id: get_bytes(&mut map, "id"),
            },
            DHT_FIND_NODE_QUERY_STR => Self::FindNode {
                id: get_bytes(&mut map, "id"),
                target: get_bytes(&mut map, "target"),
            },
            DHT_GET_PEERS_QUERY_STR => Self::GetPeers {
                id: get_bytes(&mut map, "id"),
                info_hash: get_bytes(&mut map, "info_hash"),
            },
            DHT_ANNOUNCE_PEER_QUERY_STR => Self::AnnouncePeer {
                id: get_bytes(&mut map, "id"),
                info_hash: get_bytes(&mut map, "info_hash"),
                port: get_i64(&mut map, "port"),
                token: get_bytes(&mut map, "token"),
                implied_port: get_i64(&mut map, "implied_port"),
            },
            _ => Self::Other(map),
        }
    }

    pub fn into_dict_args(self) -> BTreeMap<String, BencodeValue> {
        match self {
            Self::Ping { id } => BTreeMap::from([("id".into(), BencodeValue::Bytes(id))]),
            Self::FindNode { id, target } => BTreeMap::from([
                ("id".into(), BencodeValue::Bytes(id)),
                ("target".into(), BencodeValue::Bytes(target)),
            ]),
            Self::GetPeers { id, info_hash } => BTreeMap::from([
                ("id".into(), BencodeValue::Bytes(id)),
                ("info_hash".into(), BencodeValue::Bytes(info_hash)),
            ]),
            Self::AnnouncePeer {
                id,
                info_hash,
                port,
                token,
                implied_port,
            } => BTreeMap::from([
                ("id".into(), BencodeValue::Bytes(id)),
                ("info_hash".into(), BencodeValue::Bytes(info_hash)),
                ("port".into(), BencodeValue::Int(port)),
                ("token".into(), BencodeValue::Bytes(token)),
                ("implied_port".into(), BencodeValue::Int(implied_port)),
            ]),
            Self::Other(d) => d,
            _ => BTreeMap::default(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Ping { .. } => DHT_PING_QUERY_STR,
            Self::FindNode { .. } => DHT_FIND_NODE_QUERY_STR,
            Self::GetPeers { .. } => DHT_GET_PEERS_QUERY_STR,
            Self::AnnouncePeer { .. } => DHT_ANNOUNCE_PEER_QUERY_STR,
            _ => "",
        }
    }
}

#[inline]
fn get_bytes(map: &mut BTreeMap<String, BencodeValue>, key: &str) -> Vec<u8> {
    match map.remove(key) {
        Some(BencodeValue::Bytes(b)) => b,
        _ => Vec::default(),
    }
}

#[inline]
fn get_i64(map: &mut BTreeMap<String, BencodeValue>, key: &str) -> i64 {
    match map.remove(key) {
        Some(BencodeValue::Int(i)) => i,
        _ => 0,
    }
}

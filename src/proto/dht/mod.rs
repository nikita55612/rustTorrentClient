use super::constants::DNT_CLIENT_VERSION;
use crate::{error::Result, torrent::infohash::InfoHash};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct KrpcMessage {
    #[serde(default, with = "serde_bytes")]
    t: Vec<u8>,

    y: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    q: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    a: Option<BTreeMap<String, BencodeValue>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    r: Option<BTreeMap<String, BencodeValue>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    e: Option<(i64, String)>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    v: Option<BencodeValue>,
}

impl KrpcMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_bencode::from_bytes(bytes)?)
    }

    pub fn build_ping_query(transaction_id: &[u8], node_id: &[u8; 20]) -> KrpcMessage {
        let a = BTreeMap::from([("id".into(), BencodeValue::Bytes(node_id.into()))]);

        KrpcMessage {
            t: transaction_id.to_vec(),
            y: "q".into(),
            q: Some("ping".into()),
            a: Some(a),
            v: Some(BencodeValue::Bytes(DNT_CLIENT_VERSION.into())),
            ..Default::default()
        }
    }

    pub fn build_get_peers_query(
        transaction_id: &[u8],
        node_id: &[u8; 20],
        info_hash: &InfoHash,
    ) -> KrpcMessage {
        let a = BTreeMap::from([
            ("id".into(), BencodeValue::Bytes(node_id.into())),
            (
                "info_hash".into(),
                BencodeValue::Bytes(info_hash.inner().truncated_bytes().into()),
            ),
        ]);

        KrpcMessage {
            t: transaction_id.to_vec(),
            y: "q".into(),
            q: Some("get_peers".into()),
            a: Some(a),
            v: Some(BencodeValue::Bytes(DNT_CLIENT_VERSION.into())),
            ..Default::default()
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_bencode::to_bytes(&self).unwrap()
    }
}

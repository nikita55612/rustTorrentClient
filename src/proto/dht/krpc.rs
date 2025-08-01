/// <https://bittorrent.org/beps/bep_0005.html#krpc-protocol>
use super::{QueryArgs, ResponseArgs};
use crate::error::{Error, Result};
use crate::proto::constants::DHT_CLIENT_VERSION;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicI32, Ordering};

pub type DhtTransactionID = i32;

static GLOBAL_TID_COUNTER: AtomicI32 = AtomicI32::new(42);

pub fn fetch_add_dht_transaction_id() -> DhtTransactionID {
    GLOBAL_TID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Default, Clone)]
pub enum KrpcArgs {
    Query(QueryArgs),
    Response(ResponseArgs),
    Error((i64, String)),

    #[default]
    None,
}

impl KrpcArgs {
    pub fn from_message(mut msg: KrpcMessage) -> Self {
        let query = msg.q.as_deref().unwrap_or("");
        match msg.y.as_str() {
            "q" => Self::Query(QueryArgs::parse(query, msg.a.take())),
            "r" => Self::Response(ResponseArgs::parse(query, msg.r.take())),
            "e" => Self::Error(msg.e.take().unwrap_or_default()),
            _ => Self::None,
        }
    }
}

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
    v: Option<Vec<u8>>,
}

impl KrpcMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_bencode::from_bytes::<Self>(bytes)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_bencode::to_bytes(&self)?)
    }

    pub fn transaction_id(&self) -> Result<DhtTransactionID> {
        Ok(i32::from_be_bytes(
            self.t
                .as_slice()
                .try_into()
                .map_err(|_| Error::InvalidKrpcDhtTransactionID)?,
        ))
    }

    pub fn version(&self) -> Option<&[u8]> {
        self.v.as_deref()
    }

    pub fn into_args(mut self, q: Option<&str>) -> KrpcArgs {
        if q.is_some() {
            self.q = q.map(String::from);
        }
        KrpcArgs::from_message(self)
    }

    pub fn from_query_args(transaction_id: &DhtTransactionID, q_args: QueryArgs) -> Self {
        Self {
            t: transaction_id.to_be_bytes().to_vec(),
            y: "q".into(),
            q: Some(q_args.as_str().into()),
            a: Some(q_args.into_dict_args()),
            v: Some(DHT_CLIENT_VERSION.into()),
            ..Default::default()
        }
    }
}

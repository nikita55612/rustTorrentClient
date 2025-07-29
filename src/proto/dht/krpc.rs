/// <https://bittorrent.org/beps/bep_0005.html#krpc-protocol>
use super::{QueryArgs, ResponseArgs};
use crate::error::{Error, Result};
use crate::proto::constants::{DHT_CLIENT_VERSION, DHT_TRANSACTION_ID_SIZE};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Deref;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DhtTransactionID([u8; DHT_TRANSACTION_ID_SIZE]);

impl Deref for DhtTransactionID {
    type Target = [u8; DHT_TRANSACTION_ID_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DhtTransactionID {
    pub fn new(buf: [u8; DHT_TRANSACTION_ID_SIZE]) -> Self {
        Self(buf)
    }

    pub fn gen_new() -> Self {
        Self(rand::random())
    }
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
    v: Option<BencodeValue>,
}

impl KrpcMessage {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_bencode::from_bytes::<Self>(bytes)?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_bencode::to_bytes(&self)?)
    }

    pub fn transaction_id(&self) -> Result<DhtTransactionID> {
        Ok(DhtTransactionID::new(
            self.t
                .as_slice()
                .try_into()
                .map_err(|_| Error::InvalidKrpcDhtTransactionID)?,
        ))
    }

    pub fn version(&self) -> Option<&[u8]> {
        match &self.v {
            Some(BencodeValue::Bytes(b)) => Some(b.as_slice()),
            _ => None,
        }
    }

    pub fn into_args(mut self, q: Option<&str>) -> KrpcArgs {
        if q.is_some() {
            self.q = q.map(String::from);
        }
        KrpcArgs::from_message(self)
    }

    pub fn from_query_args(transaction_id: &DhtTransactionID, q_args: QueryArgs) -> Self {
        Self {
            t: transaction_id.to_vec(),
            y: "q".into(),
            q: Some(q_args.as_str().into()),
            a: Some(q_args.into_dict_args()),
            v: Some(BencodeValue::Bytes(DHT_CLIENT_VERSION.into())),
            ..Default::default()
        }
    }
}

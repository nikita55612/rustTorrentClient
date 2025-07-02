/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
use crate::error::Result;
use crate::metadata::info::Info;
use crate::types::infohash::{InfoHash, InfoHashV1};
use serde::Deserialize;

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MetaData {
    #[serde(default, rename = "info")]
    info_dict: Option<BencodeValue>,

    #[serde(skip)]
    pub info_bytes: Vec<u8>,

    #[serde(default)]
    pub announce: String,

    #[serde(default)]
    pub announce_list: Vec<Vec<String>>,
    //
    // #[serde(default)]
    // pub nodes: Vec<Node>,
    //
    #[serde(default, rename = "creation date")]
    pub creation_date: Option<i64>,

    #[serde(default)]
    pub comment: Option<String>,

    #[serde(default, rename = "created by")]
    pub created_by: Option<String>,

    #[serde(default)]
    pub encoding: Option<String>,
}

impl MetaData {
    pub fn from_file(path: &str) -> Result<Self> {
        Self::from_bytes(&std::fs::read(path)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut metadata = serde_bencode::from_bytes::<Self>(bytes)?;

        let info_dict = metadata.info_dict.take();
        if let Some(info_dict) = info_dict {
            metadata.info_bytes = serde_bencode::ser::to_bytes(&info_dict)?;
        }

        Ok(metadata)
    }

    pub fn info_hash(&self) -> impl InfoHash {
        InfoHashV1::from_bytes(&self.info_bytes)
    }

    pub fn deserialize_iinfo_bytes(&self) -> Result<Info> {
        Ok(serde_bencode::from_bytes(&self.info_bytes)?)
    }
}

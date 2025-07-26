/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
use super::info::Info;
use crate::error::Result;
use crate::torrent::infohash::{InfoHash, InfoHashV1, InfoHashV2};
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};

type BencodeValue = serde_bencode::value::Value;

pub type AnnounceList = Vec<Vec<String>>;

pub type Nodes = Vec<SocketAddr>;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MetaInfo {
    // info_value -> info_bytes -> info
    #[serde(default, rename = "info")]
    info_value: Option<BencodeValue>,

    // info_bytes -> info_hash
    #[serde(skip)]
    info_bytes: Option<Vec<u8>>,

    #[serde(skip)]
    pub info: Info,

    // announce -> announce_list
    #[serde(default)]
    announce: Option<String>,

    #[serde(default)]
    announce_list: Option<AnnounceList>,

    // nodes_value -> nodes
    #[serde(default, rename = "nodes")]
    nodes_value: Option<Vec<[String; 2]>>,

    #[serde(skip)]
    pub nodes: Option<Nodes>,

    #[serde(default, rename = "url-list")]
    pub url_list: Option<Vec<String>>,

    #[serde(default, rename = "creation date")]
    pub creation_date: Option<i64>,

    #[serde(default)]
    pub comment: Option<String>,

    #[serde(default, rename = "created by")]
    pub created_by: Option<String>,

    #[serde(default)]
    pub encoding: Option<String>,
}

impl MetaInfo {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut metainfo = serde_bencode::from_bytes::<Self>(bytes)?;

        if let Some(info_value) = metainfo.info_value.take() {
            let info_bytes = serde_bencode::ser::to_bytes(&info_value)?;
            metainfo.info = Info::from_bytes(&info_bytes)?;
            metainfo.info_bytes.replace(info_bytes);
        }
        if let Some(nodes_value) = metainfo.nodes_value.take() {
            let nodes = nodes_value
                .into_iter()
                .map(|v| {
                    let ip = v[0].parse::<IpAddr>();
                    let port = v[1].parse::<u16>();
                    if ip.is_ok() && port.is_ok() {
                        Some(SocketAddr::new(ip.unwrap(), port.unwrap()))
                    } else {
                        None
                    }
                })
                .filter(|v| v.is_some())
                .map(Option::unwrap)
                .collect::<Vec<_>>();
            metainfo.nodes.replace(nodes);
        }

        Ok(metainfo)
    }

    pub fn take_info_hash(&mut self) -> Option<InfoHash> {
        self.info_bytes.take().map(|info_bytes| {
            if self.info.meta_version() == 2 {
                return InfoHash::V2(InfoHashV2::from_bytes(&info_bytes));
            }
            InfoHash::V1(InfoHashV1::from_bytes(&info_bytes))
        })
    }

    pub fn take_announce_list(&mut self) -> Option<AnnounceList> {
        if let Some(announce_list) = self.announce_list.take() {
            return Some(announce_list);
        }
        if let Some(announce) = self.announce.take() {
            let mut announce_list = Vec::with_capacity(1);
            let tier = vec![announce; 1];
            announce_list.push(tier);
            return Some(announce_list);
        }
        None
    }
}

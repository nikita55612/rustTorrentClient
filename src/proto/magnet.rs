/// <https://bittorrent.org/beps/bep_0009.html#magnet-uri-format>
use crate::error::Error;
use crate::proto::constants::{INFO_HASH_V1_HEX_SIZE, INFO_HASH_V2_HEX_SIZE};
use crate::proto::infohash::{InfoHash, InfoHashV1, InfoHashV2};
use reqwest::Url;
use std::net::SocketAddr;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MagnetLink {
    pub info_hash: InfoHash,
    pub display_name: Option<String>,
    pub trackers: Vec<String>,
    pub peers: Vec<SocketAddr>,
}

impl FromStr for MagnetLink {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let url = Url::parse(s).map_err(|e| Error::ParseMagnetLink(format!("Invalid URL: {e}")))?;
        if url.scheme() != "magnet" {
            return Err(Error::ParseMagnetLink("URI scheme is not 'magnet'".into()));
        }

        let mut info_hash = None;
        let mut display_name = None;
        let mut trackers = Vec::new();
        let mut peers = Vec::<SocketAddr>::new();

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "xt" if value.starts_with("urn:btih:") => {
                    let hash = &value["urn:btih:".len()..];
                    info_hash = Some(match hash.len() {
                        32 => {
                            let base32 = hash.to_uppercase();
                            InfoHashV1::from_base32(&base32)
                                .map(InfoHash::V1)
                                .map_err(|e| {
                                    Error::ParseMagnetLink(format!("Invalid base32 btih: {e}"))
                                })
                        }
                        INFO_HASH_V1_HEX_SIZE => {
                            let bytes: [u8; INFO_HASH_V1_HEX_SIZE] =
                                hash.as_bytes().try_into().map_err(|_| {
                                    Error::ParseMagnetLink("btih hex length mismatch".into())
                                })?;
                            InfoHashV1::from_hex(&bytes).map(InfoHash::V1).map_err(|e| {
                                Error::ParseMagnetLink(format!("Invalid hex btih: {e}"))
                            })
                        }
                        _ => Err(Error::ParseMagnetLink(format!("Unknown btih format: {s}"))),
                    }?);
                }
                "xt" if value.starts_with("urn:btmh:") => {
                    let hash = &value["urn:btmh:".len()..];
                    if hash.len() == INFO_HASH_V2_HEX_SIZE {
                        let v2 = InfoHashV2::from_hex(hash.as_bytes().try_into().unwrap())
                            .map_err(|e| Error::ParseMagnetLink(format!("Invalid btmh: {e}")))?;
                        info_hash = Some(InfoHash::V2(v2));
                    }
                }
                "dn" => {
                    if display_name.is_none() {
                        display_name = Some(value.to_string());
                    }
                }
                "tr" => {
                    trackers.push(value.to_string());
                }
                "x.pe" => {
                    if let Ok(addr) = value.parse() {
                        peers.push(addr);
                    }
                }
                _ => {}
            }
        }

        let info_hash = info_hash
            .ok_or_else(|| Self::Err::ParseMagnetLink("No infohash (xt) parameter found".into()))?;

        Ok(Self {
            info_hash,
            display_name,
            trackers,
            peers,
        })
    }
}

impl MagnetLink {}

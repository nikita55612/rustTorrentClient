/// https://bittorrent.org/beps/bep_0009.html#magnet-uri-format
use crate::error::Error;
use crate::torrent::infohash::{InfoHash, InfoHashV1, InfoHashV2};
use reqwest::Url;
use std::net::{AddrParseError, SocketAddr};
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
        let url = Url::parse(s).map_err(|err| Self::Err::ParseMagnetLink(err.to_string()))?;
        if url.scheme() != "magnet" {
            return Err(Self::Err::ParseMagnetLink(
                "URI scheme is not magnet".into(),
            ));
        }

        let mut info_hash = None;
        let mut display_name = None;
        let mut trackers = Vec::new();
        let mut peers = Vec::<SocketAddr>::new();

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "xt" if value.starts_with("urn:btih:") => {
                    let data = value.trim_start_matches("urn:btih:");
                    if data.len() == 32 {
                        info_hash =
                            Some(InfoHash::V1(InfoHashV1::from_base32(&data.to_uppercase())?));
                    } else {
                        info_hash = Some(InfoHash::V1(InfoHashV1::from_hex(data)?));
                    }
                }
                "xt" if value.starts_with("urn:btmh:") => {
                    let hex = value.trim_start_matches("urn:btmh:");
                    info_hash = Some(InfoHash::V2(InfoHashV2::from_hex(hex)?));
                }
                "dn" => {
                    display_name = Some(value.to_string());
                }
                "tr" => {
                    trackers.push(value.to_string());
                }
                "x.pe" => {
                    peers.push(value.parse().map_err(|err: AddrParseError| {
                        Self::Err::ParseMagnetLink(err.to_string())
                    })?);
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

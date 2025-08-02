use crate::{
    error::{Error, Result},
    proto::MagnetLink,
    torrent::{
        infohash::{
            InfoHash, InfoHashV1, InfoHashV2, INFO_HASH_V1_HEX_SIZE, INFO_HASH_V2_HEX_SIZE,
        },
        MetaInfo, TorrentID,
    },
};
use reqwest::Url;
use std::str::FromStr;
use tokio::fs;

#[derive(Debug, Clone)]
pub enum TorrentSource {
    File(MetaInfo),
    Magnet(MagnetLink),
    InfoHash(InfoHash),
}

impl TorrentSource {
    pub fn split_torrent_id(self) -> (Self, TorrentID) {
        let torrent_id = self.torrent_id();
        (self, torrent_id)
    }

    pub fn torrent_id(&self) -> TorrentID {
        *match self {
            Self::File(meta_info) => meta_info.info_hash(),
            Self::Magnet(magnet_link) => &magnet_link.info_hash,
            Self::InfoHash(info_hash) => info_hash,
        }
        .inner()
        .truncate()
    }

    pub async fn from_str(s: &str) -> Result<Self> {
        fn parse_metainfo_bytes(bytes: &[u8]) -> Result<MetaInfo> {
            MetaInfo::from_bytes(bytes)
                .map_err(|e| Error::ParseTorrentSource(format!("Invalid torrent metadata: {e}")))
        }

        let s = s.trim();

        if let Ok(url) = Url::parse(s) {
            return match url.scheme() {
                "magnet" => MagnetLink::from_str(s)
                    .map(Self::Magnet)
                    .map_err(|e| Error::ParseTorrentSource(format!("Invalid magnet link: {e}"))),
                "http" | "https" => {
                    let response = reqwest::get(url)
                        .await
                        .map_err(|e| Error::ParseTorrentSource(format!("Request failed: {e}")))?;

                    let bytes = response.bytes().await.map_err(|e| {
                        Error::ParseTorrentSource(format!("Failed to read response: {e}"))
                    })?;

                    parse_metainfo_bytes(&bytes).map(Self::File)
                }
                scheme => Err(Error::ParseTorrentSource(format!(
                    "Unsupported URL scheme: {scheme}"
                ))),
            };
        }

        match s.len() {
            INFO_HASH_V1_HEX_SIZE => InfoHashV1::from_hex(s.as_bytes().try_into().unwrap())
                .map(|v1| Self::InfoHash(InfoHash::V1(v1)))
                .map_err(|e| Error::ParseTorrentSource(format!("Invalid v1 infohash: {e}"))),
            INFO_HASH_V2_HEX_SIZE => InfoHashV2::from_hex(s.as_bytes().try_into().unwrap())
                .map(|v2| Self::InfoHash(InfoHash::V2(v2)))
                .map_err(|e| Error::ParseTorrentSource(format!("Invalid v2 infohash: {e}"))),
            _ => {
                if let Ok(metadata) = fs::metadata(s).await {
                    if metadata.is_file() {
                        let buf = fs::read(s).await.map_err(|e| {
                            Error::ParseTorrentSource(format!("Failed to read file: {e}"))
                        })?;
                        return parse_metainfo_bytes(&buf).map(Self::File);
                    }
                }

                Err(Error::ParseTorrentSource(
                    "Unrecognized torrent source: expected magnet link, URL, file path, or hex infohash.".into(),
                ))
            }
        }
    }
}

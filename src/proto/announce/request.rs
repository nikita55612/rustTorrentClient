/// https://wiki.theory.org/BitTorrentSpecification#Tracker_Request_Parameters
/// https://bittorrent.org/beps/bep_0015.html#announce
use super::AnnounceBuilder;
use crate::proto::{announce::Event, infohash::InfoHash, PeerId};

#[derive(Debug, Clone)]
pub struct AnnounceRequestParams<'a> {
    pub info_hash: &'a InfoHash,
    pub peer_id: &'a PeerId,
    pub port: u16,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
    pub compact: Option<u8>,
    pub no_peer_id: Option<u8>,
    pub event: Option<Event>,
    pub ip: Option<String>,
    pub numwant: Option<u32>,
    pub key: Option<String>,
    pub tracker_id: Option<String>,
}

impl<'a> AnnounceRequestParams<'a> {
    pub fn builder() -> AnnounceBuilder<'a> {
        AnnounceBuilder::new()
    }

    pub fn to_bep15_bytes(&self, connection_id: u64, transaction_id: u32, key: u32) -> [u8; 98] {
        let mut buf = [0u8; 98];
        let mut offset = 0;

        buf[offset..offset + 8].copy_from_slice(&connection_id.to_be_bytes());
        offset += 8;

        buf[offset..offset + 4].copy_from_slice(&[0, 0, 0, 1]);
        offset += 4;

        buf[offset..offset + 4].copy_from_slice(&transaction_id.to_be_bytes());
        offset += 4;

        buf[offset..offset + 20].copy_from_slice(self.info_hash.inner().truncate());
        offset += 20;

        buf[offset..offset + 20].copy_from_slice(self.peer_id.as_slice());
        offset += 20;

        buf[offset..offset + 8].copy_from_slice(&self.downloaded.to_be_bytes());
        offset += 8;

        buf[offset..offset + 8].copy_from_slice(&self.left.to_be_bytes());
        offset += 8;

        buf[offset..offset + 8].copy_from_slice(&self.uploaded.to_be_bytes());
        offset += 8;

        buf[offset..offset + 4]
            .copy_from_slice(&self.event.as_ref().map_or(0, |e| e.as_int()).to_be_bytes());
        offset += 4;

        let ip = self
            .ip
            .as_ref()
            .and_then(|s| s.parse::<std::net::Ipv4Addr>().ok())
            .map(|ip| u32::from(ip))
            .unwrap_or(0);
        buf[offset..offset + 4].copy_from_slice(&ip.to_be_bytes());
        offset += 4;

        buf[offset..offset + 4].copy_from_slice(&key.to_be_bytes());
        offset += 4;

        let numwant = self.numwant.unwrap_or(u32::MAX);
        buf[offset..offset + 4].copy_from_slice(&numwant.to_be_bytes());
        offset += 4;

        buf[offset..offset + 2].copy_from_slice(&self.port.to_be_bytes());

        buf
    }

    pub fn to_query_string(&self) -> String {
        let mut query = Vec::new();

        query.push(format!("info_hash={}", self.info_hash.inner().urlencode()));
        query.push(format!("peer_id={}", self.peer_id.urlencode()));
        query.push(format!("port={}", self.port));
        query.push(format!("uploaded={}", self.uploaded));
        query.push(format!("downloaded={}", self.downloaded));
        query.push(format!("left={}", self.left));

        if let Some(val) = self.compact {
            query.push(format!("compact={}", val));
        }

        if let Some(val) = self.no_peer_id {
            query.push(format!("no_peer_id={}", val));
        }

        if let Some(ref val) = self.event {
            query.push(format!("event={}", val.as_str()));
        }

        if let Some(ref val) = self.ip {
            query.push(format!("ip={}", val));
        }

        if let Some(val) = self.numwant {
            query.push(format!("numwant={}", val));
        }

        if let Some(ref val) = self.key {
            query.push(format!("key={}", val));
        }

        if let Some(ref val) = self.tracker_id {
            query.push(format!("trackerid={}", val));
        }

        query.join("&")
    }
}

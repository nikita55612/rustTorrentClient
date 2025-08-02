use super::AnnounceRequestParams;
use crate::{
    error::{Error, Result},
    proto::{announce::Event, infohash::InfoHash, PeerId},
};

#[derive(Debug, Default, Clone)]
pub struct AnnounceBuilder<'a> {
    info_hash: Option<&'a InfoHash>,
    peer_id: Option<&'a PeerId>,
    port: Option<u16>,
    uploaded: Option<u64>,
    downloaded: Option<u64>,
    left: Option<u64>,
    compact: Option<u8>,
    no_peer_id: Option<u8>,
    event: Option<Event>,
    ip: Option<String>,
    numwant: Option<u32>,
    key: Option<String>,
    tracker_id: Option<String>,
}

impl<'a> AnnounceBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn info_hash(mut self, value: &'a InfoHash) -> Self {
        self.info_hash = Some(value);
        self
    }

    pub fn peer_id(mut self, value: &'a PeerId) -> Self {
        self.peer_id = Some(value);
        self
    }

    pub fn port(mut self, value: u16) -> Self {
        self.port = Some(value);
        self
    }

    pub fn uploaded(mut self, value: u64) -> Self {
        self.uploaded = Some(value);
        self
    }

    pub fn downloaded(mut self, value: u64) -> Self {
        self.downloaded = Some(value);
        self
    }

    pub fn left(mut self, value: u64) -> Self {
        self.left = Some(value);
        self
    }

    pub fn compact(mut self, value: u8) -> Self {
        self.compact = Some(value);
        self
    }

    pub fn no_peer_id(mut self, value: u8) -> Self {
        self.no_peer_id = Some(value);
        self
    }

    pub fn event(mut self, value: Event) -> Self {
        self.event = Some(value);
        self
    }

    pub fn ip<S: Into<String>>(mut self, value: S) -> Self {
        self.ip = Some(value.into());
        self
    }

    pub fn numwant(mut self, value: u32) -> Self {
        self.numwant = Some(value);
        self
    }

    pub fn key<S: Into<String>>(mut self, value: S) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn tracker_id<S: Into<String>>(mut self, value: S) -> Self {
        self.tracker_id = Some(value.into());
        self
    }

    pub fn build(self) -> Result<AnnounceRequestParams<'a>> {
        Ok(AnnounceRequestParams {
            info_hash: self
                .info_hash
                .ok_or_else(|| Error::AnnounceBuilder("info_hash is required".into()))?,
            peer_id: self
                .peer_id
                .ok_or_else(|| Error::AnnounceBuilder("peer_id is required".into()))?,
            port: self
                .port
                .ok_or_else(|| Error::AnnounceBuilder("port is required".into()))?,
            uploaded: self
                .uploaded
                .ok_or_else(|| Error::AnnounceBuilder("uploaded is required".into()))?,
            downloaded: self
                .downloaded
                .ok_or_else(|| Error::AnnounceBuilder("downloaded is required".into()))?,
            left: self
                .left
                .ok_or_else(|| Error::AnnounceBuilder("left is required".into()))?,
            compact: self.compact,
            no_peer_id: self.no_peer_id,
            event: self.event,
            ip: self.ip,
            numwant: self.numwant,
            key: self.key,
            tracker_id: self.tracker_id,
        })
    }
}

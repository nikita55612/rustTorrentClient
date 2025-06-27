use crate::{
    bencode::{InfoHash, TrackerResponse},
    error::Result,
    peer::PeerId,
};

const DEFAULT_PORT: u16 = 5666;
const DEFAULT_LEFT: usize = 0;
const DEFAULT_UPLOADED: usize = 0;
const DEFAULT_DOWNLOADED: usize = 0;
const DEFAULT_EVENT: Event = Event::Started;
const USER_AGENT: &'static str = "uTorrent/2210(25110)";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Started,
    Completed,
    Stopped,
}

impl Event {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Started => "started",
            Self::Completed => "completed",
            Self::Stopped => "stopped",
        }
    }
}

#[derive(Debug)]
pub struct TrackerRequest {
    pub info_hash: InfoHash,
    pub peer_id: PeerId,
    pub port: u16,
    pub left: usize,
    pub uploaded: usize,
    pub downloaded: usize,
    pub event: Event,
    pub compact: bool,
}

impl Default for TrackerRequest {
    fn default() -> Self {
        Self {
            info_hash: InfoHash::new([0u8; 20]),
            peer_id: PeerId::new([0u8; 20]),
            port: DEFAULT_PORT,
            left: DEFAULT_LEFT,
            uploaded: DEFAULT_UPLOADED,
            downloaded: DEFAULT_DOWNLOADED,
            event: DEFAULT_EVENT,
            compact: true,
        }
    }
}

impl TrackerRequest {
    pub fn with_info_hash(mut self, info_hash: &InfoHash) -> Self {
        self.info_hash = info_hash.clone();
        self
    }

    pub fn with_peer_id(mut self, peer_id: &PeerId) -> Self {
        self.peer_id = peer_id.clone();
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_left(mut self, left: usize) -> Self {
        self.left = left;
        self
    }

    pub fn with_uploaded(mut self, uploaded: usize) -> Self {
        self.uploaded = uploaded;
        self
    }

    pub fn with_downloaded(mut self, downloaded: usize) -> Self {
        self.downloaded = downloaded;
        self
    }

    pub fn with_event(mut self, event: Event) -> Self {
        self.event = event;
        self
    }

    pub fn not_compact(mut self) -> Self {
        self.compact = false;
        self
    }

    pub async fn make(&self, announce: &str) -> Result<TrackerResponse> {
        let client = reqwest::Client::default();
        let url = format!(
            "{}?{}&{}&{}&{}&{}&{}&{}&{}",
            announce,
            format!("info_hash={}", self.info_hash.percent_encoding()),
            format!("peer_id={}", String::from_utf8_lossy(self.peer_id.bytes()),),
            format!("port={}", self.port),
            format!("uploaded={}", self.uploaded),
            format!("downloaded={}", self.downloaded),
            format!("left={}", self.left),
            format!("event={}", self.event.as_str()),
            format!("compact={}", self.compact as u8),
        );
        let bytes = client
            .get(url)
            .timeout(std::time::Duration::from_secs(5))
            .header("User-Agent", USER_AGENT)
            .send()
            .await?
            .bytes()
            .await?;

        Ok(TrackerResponse::from_bytes(&bytes)?)
    }
}

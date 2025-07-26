use std::{
    ops::Deref,
    time::{Duration, Instant, SystemTime},
};
use tokio::sync::Mutex;

use crate::{
    proto::{BitField, PeerId},
    torrent::{
        infohash::InfoHash,
        state::{TorrentProgress, TorrentStatus},
        AnnounceList, TorrentTrackerState,
    },
};

#[derive(Debug, Clone)]
pub struct TorrentInitStateParams {
    pub status: TorrentStatus,
    pub port: u16,
    pub peer_id: PeerId,
    pub info_hash: InfoHash,
    pub announce_list: AnnounceList,
    pub num_pieces: usize,
    pub bitfield: Option<BitField>,
    pub have_pieces: usize,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
}

#[derive(Debug)]
pub struct TorrentState {
    status: TorrentStatus,
    pub port: u16,
    pub peer_id: PeerId,
    pub info_hash: InfoHash,
    pub announce_list: AnnounceList,
    pub bitfield: BitField,
    pub tracker: TorrentTrackerState,
    pub progress: TorrentProgress,
    pub num_seeders: u32,
    pub num_leechers: u32,
    download_start_time: Option<Instant>,
    active_download_duration: Duration,
    pub init_time: SystemTime,
    mu: Mutex<()>,
}

impl Deref for TorrentState {
    type Target = Mutex<()>;

    fn deref(&self) -> &Self::Target {
        &self.mu
    }
}

impl TorrentState {
    pub fn init(params: TorrentInitStateParams) -> Self {
        let TorrentInitStateParams {
            status,
            port,
            peer_id,
            info_hash,
            announce_list,
            num_pieces,
            bitfield,
            have_pieces,
            uploaded,
            downloaded,
            left,
        } = params;
        let download_start_time = match status {
            TorrentStatus::Downloading => Some(Instant::now()),
            _ => None,
        };
        Self {
            status,
            port,
            peer_id,
            info_hash,
            announce_list,
            bitfield: bitfield.unwrap_or(BitField::new(num_pieces)),
            tracker: TorrentTrackerState::default(),
            progress: TorrentProgress {
                num_pieces,
                have_pieces,
                uploaded,
                downloaded,
                left,
            },
            num_seeders: 0,
            num_leechers: 0,
            download_start_time,
            active_download_duration: Duration::ZERO,
            init_time: SystemTime::now(),
            mu: Mutex::new(()),
        }
    }

    pub fn status(&self) -> TorrentStatus {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: TorrentStatus) {
        match &status {
            &TorrentStatus::Downloading
                if self.status != TorrentStatus::Downloading
                    && self.download_start_time.is_none() =>
            {
                self.download_start_time = Some(Instant::now());
            }
            s if *s != TorrentStatus::Downloading && self.download_start_time.is_some() => {
                if let Some(start_time) = self.download_start_time.take() {
                    self.active_download_duration += start_time.elapsed();
                }
            }
            _ => {}
        }

        self.status = status;
    }

    pub fn on_downloaded(&mut self, num_bytes: u64) {
        self.progress.downloaded += num_bytes;
        self.progress.left -= num_bytes;
    }

    pub fn on_uploaded(&mut self, num_bytes: u64) {
        self.progress.uploaded += num_bytes;
    }

    pub fn active_download_time(&self) -> Duration {
        match self.download_start_time {
            Some(start) => self.active_download_duration + start.elapsed(),
            None => self.active_download_duration,
        }
    }

    fn build_params(&self, event: Event) -> AnnounceRequestParams {
        let mut params = AnnounceBuilder::new()
            .info_hash(self.info_hash.inner().urlencode())
            .peer_id(self.peer_id.urlencode())
            .port(self.port)
            .uploaded(ctx_data.state.progress.uploaded)
            .downloaded(ctx_data.state.progress.downloaded)
            .left(ctx_data.state.progress.left)
            .event(event.as_str())
            .build()
            .unwrap();
        if let Some(tracker_id) = self.tracker_id.as_ref() {
            params.tracker_id = Some(tracker_id.clone());
        }
        params
    }

    pub async fn announce_completed(&self) {
        if let Some(cli) = self.tracker.client.as_ref() {
            let params = self.build_params(Event::Completed, ctx_data);
            let _ = cli.announce(&params).await;
        }
    }

    pub async fn announce_stopped(&self) {
        if let Some(cli) = self.client.as_ref() {
            let params = self.build_params(Event::Stopped, ctx_data);
            let _ = cli.announce(&params).await;
        }
    }
}

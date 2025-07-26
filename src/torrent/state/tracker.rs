use std::{ops::Deref, sync::Mutex};

use crate::{
    torrent::{TorrentCtx, TorrentCtxData},
    tracker::{
        announce::{AnnounceBuilder, AnnounceRequestParams, Event},
        TrackerClient,
    },
};

#[derive(Debug, Default)]
pub struct TorrentTrackerState {
    pub client: Option<TrackerClient>,
    pub url: String,
    pub id: Option<String>,
    mu: Mutex<()>,
}

impl Deref for TorrentTrackerState {
    type Target = Mutex<()>;

    fn deref(&self) -> &Self::Target {
        &self.mu
    }
}

impl TorrentTrackerState {
    async fn spawn_periodic_announce(ctx: &TorrentCtx) {}

    fn build_params(&self, event: Event, ctx_data: &TorrentCtxData) -> AnnounceRequestParams {
        let mut params = AnnounceBuilder::new()
            .info_hash(ctx_data.info_hash.inner().urlencode())
            .peer_id(ctx_data.peer_id.urlencode())
            .port(ctx_data.port)
            .uploaded(ctx_data.state.progress.uploaded)
            .downloaded(ctx_data.state.progress.downloaded)
            .left(ctx_data.state.progress.left)
            .event(event.as_str())
            .build()
            .unwrap();
        if let Some(tracker_id) = self.id.as_ref() {
            params.tracker_id = Some(tracker_id.clone());
        }
        params
    }

    pub async fn completed(&self, ctx_data: &TorrentCtxData) {
        if let Some(cli) = self.client.as_ref() {
            let params = self.build_params(Event::Completed, ctx_data);
            let _ = cli.announce(&params).await;
        }
    }

    pub async fn stopped(&self, ctx_data: &TorrentCtxData) {
        if let Some(cli) = self.client.as_ref() {
            let params = self.build_params(Event::Stopped, ctx_data);
            let _ = cli.announce(&params).await;
        }
    }
}

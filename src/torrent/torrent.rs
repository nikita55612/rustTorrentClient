use crate::{
    error::{Error, Result},
    proto::PeerId,
    util::Ctx,
};
use std::{path::Path, sync::Arc, time::Duration};
use tokio::sync::mpsc::{self, Receiver, Sender};

pub type TorrentID = [u8; 20];

// #[derive(Debug, Clone)]
// pub enum TorrentSource<'a> {
//     File(&'a [u8]),
//     Link(&'a str),
// }

// pub struct TorrentState {}

// impl TorrentState {
//     pub async fn new(source: TorrentSource) -> Self {
//         Self {}
//     }
// }

// pub type TorrentCtx = Ctx<Arc<TorrentState>>;

// #[derive(Debug)]
// pub struct TorrentCtxData {
//     pub state: TorrentState,
//     pub tracker_client_timeout: Duration,
//     pub tracker_user_agent: &'static str, // "uTorrent/2210(25110)"
// }

// конвеер
// трекер передает полученные пиры и свой адрес для отправки ивента
// // клиент треке
// pub struct Torrent {
//     pub metainfo: MetaInfo,
//     pub info_hash: InfoHash,
//     cmd_tx: Sender<String>,
//     cmd_rx: Receiver<String>,
// }

// impl Torrent {
//     pub async fn from_bytes(bytes: &[u8]) -> Result<Self> {
//         let mut metainfo = MetaInfo::from_bytes(bytes)?;
//         let info_hash = metainfo.take_info_hash().expect("");
//         let (tx, rx) = mpsc::channel(32);
//         Ok(Self {
//             metainfo: metainfo,
//             info_hash: info_hash,
//             cmd_tx: tx,
//             cmd_rx: rx,
//         })
//     }

//     pub async fn from_file(path: impl AsRef<Path>) -> Result<Self> {
//         let bytes = tokio::fs::read(path).await?;
//         Self::from_bytes(&bytes).await
//     }

//     pub async fn start_listener() {}
// }

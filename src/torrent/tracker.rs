// pub async fn available_tracker_from_announce_list(list: &AnnounceList) {
//     'outer: for tier in list {
//         for url in tier {
//             let cli = client_builder.clone().url(url).build();
//         }
//     }
// }

// available_peers: Vec<SocketAddr>,
// #[derive(Debug, Default, Clone)]
// pub struct TorrentTrackerAnnounceResponse {
//     pub peers: Peers,
// }

// #[derive(Debug, Clone)]
// pub struct TorrentTracker {
//     ctx: TorrentCtx,
// }

// 1. Начинаем загрузку
//    → event=started           → получаем список пиров

// 2. Каждые interval секунд
//    → без event               → получаем обновлённый список

// 3. Завершили загрузку
//    → event=completed         → (опционально) получаем список

// 4. Завершаем работу
//    → event=stopped           → не получаем список

// impl TorrentTracker {
//     pub fn new(ctx: TorrentCtx) -> (Self, Receiver<TorrentTrackerAnnounceResponse>) {
//         let client = Arc::new(Mutex::new(None));
//         let (tx, rx) = mpsc::channel(1);

//         let handle = tokio::spawn(Self::launch_announce(ctx.clone_data(), client.clone(), tx));

//         let mut done = ctx.done();
//         tokio::spawn(async move {
//             let _ = done.changed().await;
//             handle.abort();
//             let _ = handle.await;
//         });

//         (Self { ctx }, rx)
//     }

//     async fn launch_announce(
//         ctx_data: Arc<TorrentCtxData>,
//         tx: Sender<TorrentTrackerAnnounceResponse>,
//     ) {
//         let announce_list = &ctx_data.announce_list;
//         let num_trackers = announce_list.iter().fold(0, |acc, e| acc + e.len());
//         if num_trackers == 0 {
//             return;
//         }

//         let is_single_tracker = num_trackers <= 1;

//         // let mut tracker_state = TorrentTrackerState {
//         //     interval: Duration::new(2, 0),
//         //     ..Default::default()
//         // };

//         let mut params = AnnounceBuilder::new()
//             .info_hash(ctx_data.info_hash.inner().urlencode())
//             .peer_id(ctx_data.peer_id.urlencode())
//             .port(ctx_data.port)
//             .uploaded(0)
//             .downloaded(0)
//             .left(0)
//             // .event(Event::Started.as_str())
//             .build()
//             .unwrap();

//         let client_builder = TrackerClient::builder("")
//             .with_timeout(ctx_data.tracker_client_timeout)
//             .add_header("User-Agent", ctx_data.tracker_user_agent);

//         // интервалы
//         let mut interval = Duration::ZERO;
//         let mut min_interval = None;

//         // если при первом обновлении нет пиров, нужно обновлять чаще
//         // если подключение уже было нужно отправить announce прошлому трекеру
//         loop {
//             let guard = ctx_data.state.lock().await;
//             params.uploaded = ctx_data.state.progress.uploaded;
//             params.downloaded = ctx_data.state.progress.downloaded;
//             params.left = ctx_data.state.progress.left;
//             drop(guard);

//             if is_single_tracker {
//                 params.tracker_id = tracker_state.tracker_id.clone();
//             }

//             'outer: for tier in announce_list.iter() {
//                 for url in tier {
//                     let cli = client_builder.clone().url(url).build();
//                     if !is_single_tracker
//                         && tracker_state.tracker_id.is_some()
//                         && &tracker_state.url == url
//                     {
//                         params.tracker_id = tracker_state.tracker_id.clone();
//                     }
//                     if let Ok(Ok(mut res)) = cli.announce(&params).await.map(|r| r.ok()) {
//                         if res.peers.len() == 0 {
//                             continue;
//                         }
//                         tracker_state.url = url.clone();
//                         tracker_state.tracker_id = res.tracker_id.take();
//                         // if let Some(secs) = res.interval {
//                         //     tracker_state.interval = Duration::from_secs(secs);
//                         // }
//                         // tracker_state.min_interval = res.min_interval.map(Duration::from_secs);
//                         tracker_state.seeders = res.complete;
//                         tracker_state.leechers = res.incomplete;

//                         let _ = tx
//                             .send(TorrentTrackerAnnounceResponse { peers: res.peers })
//                             .await;

//                         client.lock().await.replace(cli);
//                         outer_state.lock().await.replace(tracker_state.clone());

//                         break 'outer;
//                     }
//                 }
//             }
//             if let Some(min_interval) = tracker_state.min_interval {
//                 let interval = (tracker_state.interval + min_interval) / 2;
//                 tokio::time::sleep(interval).await;
//             } else {
//                 tokio::time::sleep(tracker_state.interval).await;
//             }
//         }
//     }
// }

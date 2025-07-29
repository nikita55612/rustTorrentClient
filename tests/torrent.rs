use std::sync::Arc;

use rutor::proto;
use rutor::proto::constants::DHT_GET_PEERS_QUERY_STR;
use rutor::proto::dht::DhtTransactionID;
use rutor::torrent;
use tokio::net::UdpSocket;

#[tokio::test]
async fn test_metainfo() {
    let magnet = "magnet:?xt=urn:btih:f42f4f3181996ff4954dd5d7f166bc146810f8e3&dn=archlinux-2025.07.01-x86_64.iso";
    let magnet_link = magnet.parse::<proto::MagnetLink>().unwrap();

    let udp_socket = Arc::new(UdpSocket::bind(("0.0.0.0", 6675)).await.unwrap());
    let peer_id = proto::PeerId::gen_new();
    // let message = proto::dht::build_ping_query(b"r4", peer_id.as_slice().try_into().unwrap());
    // println!("{:#?}", message);
    // let buf = message.to_bytes();
    // udp_socket
    //     .send_to(&buf, proto::constants::BOOTSTRAP_NODES[0])
    //     .await;
    // let mut buf = [0u8; 2048];
    // if let Ok((n, addr)) = udp_socket.peek_from(&mut buf).await {
    //     let m = proto::dht::KrpcMessage::from_bytes(&buf[..n]).unwrap();
    //     // println!("{} -> {:#?}", addr, m);
    // }

    let message = proto::dht::KrpcMessage::from_query_args(
        &DhtTransactionID::gen_new(),
        proto::dht::QueryArgs::GetPeers {
            id: peer_id.to_vec(),
            info_hash: magnet_link.info_hash.inner().as_bytes().to_vec(),
        },
    );

    {
        for node in proto::constants::BOOTSTRAP_NODES {
            let message = message.clone();
            let udp_socket = udp_socket.clone();
            tokio::spawn(async move {
                let _ = udp_socket.send_to(&message.to_bytes().unwrap(), node).await;
            });
        }
    }

    let mut buf = [0u8; 2048];
    loop {
        if let Ok((n, addr)) = udp_socket.recv_from(&mut buf).await {
            let m = proto::dht::KrpcMessage::from_bytes(&buf[..n]).unwrap();
            println!(
                "{} -> {:?}",
                addr,
                m.into_args(Some(DHT_GET_PEERS_QUERY_STR))
            );
        }
    }

    // println!("{:#?}", magnet_link);
    // let bytes = std::fs::read("resources/archlinux-2025.07.01-x86_64.iso.torrent").unwrap();

    // let mut metainfo = torrent::MetaInfo::from_bytes(&bytes).unwrap();

    // let info_hash = metainfo.take_info_hash().unwrap();
    // println!("{}", info_hash.inner().hex());
    // println!("{:#?}", metainfo.url_list);
    // let announce_list = metainfo.take_announce_list().unwrap();
    // dbg!(&announce_list);

    // metainfo.info.
    // let

    // println!("{:#?}", metainfo.info.file_tree);
    // println!(
    //     "iter_files: {:#?}",
    //     metainfo.info.file_tree.unwrap().iter_files()
    // );
}

// #[tokio::test]
// async fn test_tracker() {
//     let bytes = std::fs::read("resources/Red_Hot_Chili_Peppers.torrent").unwrap();
//     let mut metainfo = torrent::MetaInfo::from_bytes(&bytes).unwrap();

//     let info_hash = metainfo.take_info_hash().unwrap();
//     let announce_list = Arc::new(metainfo.take_announce_list().unwrap());
//     let num_pieces = metainfo.info.num_pieces();
//     let state_params = torrent::TorrentInitStateParams {
//         bitfield: Some(proto::BitField::new(num_pieces)),
//         num_pieces,
//         left: metainfo.info.total_length(),
//         ..Default::default()
//     };
//     let state = Arc::new(torrent::TorrentState::init(state_params));

//     let ctx_data = Arc::new(torrent::TorrentCtxData {
//         port: 5554,
//         peer_id: proto::PeerId::gen_new(),
//         info_hash,
//         announce_list,
//         state,
//         tracker_user_agent: "uTorrent/2210(25110)"
//     });
//     let ctx = torrent::TorrentCtx::new(ctx_data);

//     let (tracker, mut rx) = torrent::tracker::TorrentTracker::new(ctx.clone());

//     {
//         // let ctx = ctx.clone();
//         tokio::spawn(async move {
//             tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//             println!("ctx.cancel()");
//             ctx.cancel();
//         });
//     }

//     while let Some(res) = rx.recv().await {
//         println!("{:#?}", res);
//     }
// }

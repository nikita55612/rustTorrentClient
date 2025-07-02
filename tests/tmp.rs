use std::{fs::File, io::Read};

use rutor::types::infohash::InfoHash;

pub fn percent_encoding(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(20 * 3);
    for &b in bytes {
        encoded.push('%');
        encoded.push(HEX[(b >> 4) as usize] as char);
        encoded.push(HEX[(b & 0x0F) as usize] as char);
    }
    encoded
}

#[tokio::test]
async fn test_new_metadata() {
    let mut file = File::open("resources/Red_Hot_Chili_Peppers.torrent").unwrap();

    let mut bytes_buf = Vec::new();
    file.read_to_end(&mut bytes_buf).unwrap();

    let metadata = rutor::metadata::MetaData::from_bytes(&bytes_buf).unwrap();
    let info = metadata.deserialize_iinfo_bytes().unwrap();
    let info_hash = metadata.info_hash();

    println!("torrent metadata:\n");
    println!("announce: {}", metadata.announce.as_str());
    println!("info_hash: {}", info_hash.hex());
    println!("info.name: {}", info.name.as_str());
    println!("info.piece_length: {}", info.piece_length);
    println!("info.pieces.len: {}", info.pieces.len());
    println!("info.pieces.len2: {}", info.pieces.len() / 20);
    // println!("info.length: {}", tor.info.length());
    // println!("info.files: {:#?}", tor.info.files.as_ref());

    let mut tracker_request_builder = rutor::tracker::TrackerRequest::builder();

    let tracker_request = tracker_request_builder
        .info_hash(info_hash.urlencode())
        .peer_id(rutor::types::PeerId::gen_new().urlencode())
        .port(6688)
        .left(1000)
        .downloaded(0)
        .uploaded(0)
        .build()
        .unwrap();

    let url = format!(
        "{}?{}",
        metadata.announce,
        tracker_request.to_query_string()
    );

    let client = reqwest::Client::default();
    let response = client
        .get(url)
        .header("User-Agent", "uTorrent/2210(25110)")
        .send()
        .await
        .unwrap();

    println!("{}", response.url());

    let bytes = response.bytes().await.unwrap();

    println!("{}", String::from_utf8_lossy(&bytes));

    let tracker_response = rutor::tracker::TrackerResponse::from_bytes(&bytes).unwrap();

    println!("{:#?}", tracker_response);
}

// #[tokio::test]
// async fn test_tor_meta() {
//     let mut file = File::open("resources/Red_Hot_Chili_Peppers.torrent").unwrap();

//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf).unwrap();

//     let tor = rutor::bencode::TorrentFile::deserialize(&buf).unwrap();
//     let info_hash = rutor::torrent::InfoHash::from_torrent_file_bytes(&buf).unwrap();

//     println!("torrent meta info:\n");
//     println!("announce: {}", tor.announce.as_str());
//     println!("info_hash: {}", info_hash.hex());
//     println!("info.name: {}", tor.info.name.as_str());
//     println!("info.piece_length: {}", tor.info.piece_length);
//     println!("info.pieces.len: {}", tor.info.pieces.len());
//     println!("info.pieces.len2: {}", tor.info.pieces.len() / 20);
//     // println!("info.length: {}", tor.info.length());
//     // println!("info.files: {:#?}", tor.info.files.as_ref());

//     let bit_field_data = [
//         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
//         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
//         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
//         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
//         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
//         255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
//         255, 255, 255, 128,
//     ];

//     let bit_field = rutor::proto::BitField::from(bit_field_data.as_slice());
//     println!("{}", bit_field.has_piece(888))
// }

// #[tokio::test]
// async fn test_1() {
//     let mut file = File::open("resources/Red_Hot_Chili_Peppers.torrent").unwrap();

//     let mut buf = Vec::new();
//     file.read_to_end(&mut buf).unwrap();

//     let tor = rutor::bencode::TorrentFile::from_bytes(&buf).unwrap();

//     println!("torrent:\n");
//     println!("announce: {}", tor.announce.as_str());
//     println!("info.name: {}", tor.info.name.as_str());
//     println!("info.piece_length: {}", tor.info.piece_length);
//     println!("info.length: {}", tor.info.length());
//     println!("info.files: {:#?}", tor.info.files.as_ref());
//     println!();

//     let info_hash = rutor::torrent::InfoHash::from_torrent_file_bytes(&buf).unwrap();

//     println!("info_hash: {}", info_hash.hex());

//     let peer_id = rutor::peers::PeerId::gen_new();
//     let handshake = rutor::proto::Handshake::from_parts(&info_hash, &peer_id);

//     println!("handshake: {}", String::from_utf8_lossy(handshake.bytes()));

//     let bytes = rutor::tracker::TrackerRequest::default()
//         .with_info_hash(&info_hash)
//         .with_peer_id(&peer_id)
//         .with_port(8666)
//         .with_left(tor.info.length())
//         .fetch_with_retries(&tor.announce, 3)
//         .await
//         .unwrap();
//     let res = rutor::bencode::TrackerResponse::from_bytes(&bytes).unwrap();

//     let mut peers = rutor::peers::Manager::from_bytes(&res.peers);

//     println!();
//     println!("{:#?}", peers);
//     println!();

//     // let mut jh_group = Vec::with_capacity(peers.len());

//     for (i, p) in peers.iter_mut().enumerate() {
//         let hm = rutor::proto::Message::Handshake(handshake.clone());
//         // let addr = p.addr.clone();

//         if let Err(e) = p.try_connect().await {
//             eprintln!("{}: connection failed: {:?}", i, e);
//             continue;
//         }

//         p.send(&hm).await;

//         tokio::time::sleep(std::time::Duration::from_secs(1)).await;

//         let messages = p.read_all().await;
//         println!(
//             "{}. peer messages: {:?}",
//             i,
//             messages,
//             // String::from_utf8_lossy(m.to_bytes().as_slice()),
//         );

//         tokio::time::sleep(std::time::Duration::from_secs(1)).await;

//         let messages = p.read_all().await;
//         println!(
//             "{}. peer messages: {:?}",
//             i,
//             messages,
//             // String::from_utf8_lossy(m.to_bytes().as_slice()),
//         );

//         // let conn = p.take_conn().unwrap();
//         // let mut receiver = p.take_receiver().unwrap();

//         // let jh = tokio::spawn(async move {
//         //     if let Err(e) = conn.send(&hm).await {
//         //         eprintln!("{}: send failed: {:?}", i, e);
//         //     }
//         //     while let Some(m) = receiver.recv().await {
//         //         println!(
//         //             "{}. peer message: {:?}",
//         //             i,
//         //             m,
//         //             // String::from_utf8_lossy(m.to_bytes().as_slice()),
//         //         );
//         //     }
//         //     println!("drop receiver");
//         //     conn.close().await;
//         // });

//         // jh_group.push(jh);
//     }

//     // for jh in jh_group {
//     //     let _ = jh.await;
//     // }
// }

// #[tokio::test]
// async fn test_2() {
//     rutor::storage::create_file("data", 40).await.unwrap();
//     rutor::storage::write_chunk("data", 12, b"Hello, world!")
//         .await
//         .unwrap();
// }

// #[test]
// fn test_3() {
//     use std::io::{BufReader, Read};

//     let bytes = &b"Hello, world!"[..];
//     let mut r = BufReader::new(bytes);
//     let mut buf = [0; 4];
//     r.read(&mut buf[..]).unwrap();

//     println!("{}", str::from_utf8(&buf).unwrap());
//     println!("{}", str::from_utf8(r.buffer()).unwrap());
// }

// #[test]
// fn test_4() {
//     for i in 0..10 {
//         println!("{}", i);
//     }
// }

// #[test]
// fn test_5() {
//     let p = rutor::proto::Piece::new(0, 12, vec![1, 22, 33, 33]);
//     let mb = rutor::proto::Message::Piece(p).to_bytes();
//     println!("{:?}", mb);
//     let m = rutor::proto::Message::from_bytes(&mb);
//     println!("{:?}", m);
// }

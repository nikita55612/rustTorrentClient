use std::{fs::File, io::Read};

#[tokio::test]
async fn test_1() {
    let buff = [1, 2, 3, 4, 5, 4, 5, 6, 8];
    println!("{:?}", &buff[4]);

    let mut file = File::open("/Users/Nikita/Downloads/5145412.torrent").unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let tor = rutor::bencode::Torrent::from_bytes(&buf).unwrap();

    println!("torrent:");
    println!("announce: {}", tor.announce.as_str());
    println!("info.name: {}", tor.info.name.as_str());
    println!("info.piece_length: {}", tor.info.piece_length);
    println!("info.length: {}", tor.info.length());
    println!("info.files: {:#?}", tor.info.files.as_ref());
    println!();

    let info_hash = rutor::bencode::InfoHash::from_bytes(&buf).unwrap();

    println!("info_hash: {}", info_hash.hex());

    let peer_id = rutor::peer::PeerId::gen_new();
    let handshake = rutor::proto::handshake::Handshake::from_parts(&info_hash, &peer_id);

    println!("handshake: {}", String::from_utf8_lossy(handshake.bytes()));

    let res = rutor::tracker::TrackerRequest::default()
        .with_info_hash(&info_hash)
        .with_peer_id(&peer_id)
        .with_port(8666)
        .with_left(tor.info.length())
        .make(&tor.announce)
        .await
        .unwrap();

    let peers = rutor::peer::Peers::from_bytes(&res.peers);

    println!();
    println!("{:#?}", peers);
    println!();

    for (i, p) in peers.iter().enumerate() {
        if let Ok(mut conn) = rutor::peer::conn::PeerConn::connect(&p.socket_addr).await {
            let h = conn.handshake(&handshake).await;
            if h.is_err() {
                continue;
            }
            println!(
                "{}. peer handshake: {}",
                i,
                String::from_utf8_lossy(h.unwrap().bytes())
            );
        }
    }
}

use std::{fs::File, io::Read};

#[tokio::test]
async fn test_tor_meta() {
    let mut file = File::open("resources/Red_Hot_Chili_Peppers.torrent").unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let tor = rutor::bencode::Torrent::from_bytes(&buf).unwrap();
    let info_hash = rutor::bencode::InfoHash::from_bytes(&buf).unwrap();

    println!("torrent meta info:\n");
    println!("announce: {}", tor.announce.as_str());
    println!("info_hash: {}", info_hash.hex());
    println!("info.name: {}", tor.info.name.as_str());
    println!("info.piece_length: {}", tor.info.piece_length);
    println!("info.pieces.len: {}", tor.info.pieces.len());
    println!("info.pieces.len2: {}", tor.info.pieces.len() / 20);
    println!("info.length: {}", tor.info.length());
    // println!("info.files: {:#?}", tor.info.files.as_ref());
}

#[tokio::test]
async fn test_1() {
    let mut file = File::open("resources/Red_Hot_Chili_Peppers.torrent").unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let tor = rutor::bencode::Torrent::from_bytes(&buf).unwrap();

    println!("torrent:\n");
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

    let mut jh_group = Vec::with_capacity(peers.len());

    for (i, p) in peers.iter().enumerate() {
        let hc = handshake.clone();
        let addr = p.addr.clone();

        let jh = tokio::spawn(async move {
            match rutor::peer::conn::Conn::connect(&addr).await {
                Ok((conn, mut receiver)) => {
                    // println!("task {}", i);
                    if let Err(e) = conn.send(hc.bytes()).await {
                        eprintln!("{}: send failed: {:?}", i, e);
                    }
                    while let Some(m) = receiver.recv().await {
                        println!(
                            "{}. peer message: {:?}",
                            i,
                            m,
                            // String::from_utf8_lossy(m.to_bytes().as_slice()),
                        );
                    }
                    conn.close().await;
                }
                Err(e) => {
                    eprintln!("{}: connection failed: {:?}", i, e);
                }
            }
        });

        jh_group.push(jh);
    }

    for jh in jh_group {
        let _ = jh.await;
    }
}

#[tokio::test]
async fn test_2() {
    rutor::disk::create_file("data", 40).await.unwrap();
    rutor::disk::write_chunk("data", 12, b"Hello, world!")
        .await
        .unwrap();
}

#[test]
fn test_3() {
    use std::io::{BufReader, Read};

    let bytes = &b"Hello, world!"[..];
    let mut r = BufReader::new(bytes);
    let mut buf = [0; 4];
    r.read(&mut buf[..]).unwrap();

    println!("{}", str::from_utf8(&buf).unwrap());
    println!("{}", str::from_utf8(r.buffer()).unwrap());
}

#[test]
fn test_4() {
    let u = 9u32;
    println!("{:?}", u.to_be_bytes());
}

#[test]
fn test_5() {
    let p = rutor::proto::piece::Piece::new(0, 12, vec![1, 22, 33, 33]);
    let mb = rutor::proto::message::Message::Piece(p).to_bytes();
    println!("{:?}", mb);
    let m = rutor::proto::message::Message::from_bytes(&mb);
    println!("{:?}", m);
}

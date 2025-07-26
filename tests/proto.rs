use rutor::proto;

#[test]
fn test_bitfield() {
    let mut bitfield = proto::BitField::new(10);
    println!("new bitfield: {:?}", bitfield.as_slice());
    bitfield.set(3);
    println!("set bitfield: {:?}", bitfield.as_slice());
    println!("has bitfield: {:?}", bitfield.has(3));
    println!("not has bitfield: {:?}", bitfield.has(2));
}

#[test]
fn test_peerid() {
    let peer_id = proto::PeerId::gen_new();
    println!("peer_id: {:?}", String::from_utf8_lossy(peer_id.as_slice()));
    println!(
        "header: {:?}",
        String::from_utf8_lossy(peer_id.extract_header().as_slice())
    );
}

use std::net::{Ipv4Addr, SocketAddrV4};

pub fn socket_addr_from_bytes(bytes: &[u8; 6]) -> SocketAddrV4 {
    let ip = Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]);
    let port = ((bytes[4] as u16) << 8) | (bytes[5] as u16);
    SocketAddrV4::new(ip, port)
}

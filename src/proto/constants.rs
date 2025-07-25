pub const DEFAULT_PEER_FINGERPRINT: &[u8; 8] = b"-qB5050-";

pub const MAX_MESSAGE_SIZE: usize = 2 * 1024 * 1024;

pub const CHOKE_MESS_ID: u8 = 0;
pub const UNCHOKE_MESS_ID: u8 = 1;
pub const INTERESTED_MESS_ID: u8 = 2;
pub const NOT_INTERESTED_MESS_ID: u8 = 3;
pub const HAVE_MESS_ID: u8 = 4;
pub const BITFIELD_MESS_ID: u8 = 5;
pub const REQUEST_MESS_ID: u8 = 6;
pub const PIECE_MESS_ID: u8 = 7;
pub const CANCEL_MESS_ID: u8 = 8;
pub const PORT_MESS_ID: u8 = 9;
pub const EXTENSION_MESS_ID: u8 = 20;

pub const KEEP_ALIVE_MESS: [u8; 4] = [0, 0, 0, 0];
pub const CHOKE_MESS: [u8; 5] = [0, 0, 0, 1, CHOKE_MESS_ID];
pub const UNCHOKE_MESS: [u8; 5] = [0, 0, 0, 1, UNCHOKE_MESS_ID];
pub const INTERESTED_MESS: [u8; 5] = [0, 0, 0, 1, INTERESTED_MESS_ID];
pub const NOT_INTERESTED_MESS: [u8; 5] = [0, 0, 0, 1, NOT_INTERESTED_MESS_ID];

pub const HANDSHAKE_LEN: usize = 68;
pub const HANDSHAKE_PSTR: &[u8; 19] = b"BitTorrent protocol";
pub const HANDSHAKE_PREFIX: [u8; 5] = [19, 66, 105, 116, 84];

pub const HAVE_LEN: usize = 5;
pub const REQUEST_LEN: usize = 13;
pub const PORT_LEN: usize = 3;

pub const BEP15_MAGIC_CONSTANT: [u8; 8] = [0, 0, 4, 23, 39, 16, 25, 128];
pub const BEP15_CONNECT_LEN: usize = 16;
pub const BEP15_ANNOUNCE_REQUEST_LEN: usize = 98;

pub const DNT_CLIENT_VERSION: &[u8; 4] = b"rT01";

pub const BOOTSTRAP_NODES: [&str; 6] = [
    "router.bittorrent.com:6881",
    "router.utorrent.com:6881",
    "dht.transmissionbt.com:6881",
    "dht.aelitis.com:6881",
    "router.bitcomet.com:6881",
    "dht.libtorrent.org:6881",
];

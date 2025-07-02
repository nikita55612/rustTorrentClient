pub const CHOKE_ID: u8 = 0;
pub const UNCHOKE_ID: u8 = 1;
pub const INTERESTED_ID: u8 = 2;
pub const NOT_INTERESTED_ID: u8 = 3;
pub const HAVE_ID: u8 = 4;
pub const BITFIELD_ID: u8 = 5;
pub const REQUEST_ID: u8 = 6;
pub const PIECE_ID: u8 = 7;
pub const CANCEL_ID: u8 = 8;
pub const PORT_ID: u8 = 9;

pub const KEEP_ALIVE_MESS: [u8; 4] = [0, 0, 0, 0];
pub const CHOKE_MESS: [u8; 5] = [0, 0, 0, 1, CHOKE_ID];
pub const UNCHOKE_MESS: [u8; 5] = [0, 0, 0, 1, UNCHOKE_ID];
pub const INTERESTED_MESS: [u8; 5] = [0, 0, 0, 1, INTERESTED_ID];
pub const NOT_INTERESTED_MESS: [u8; 5] = [0, 0, 0, 1, NOT_INTERESTED_ID];

pub const HANDSHAKE_LEN: usize = 68;
pub const HANDSHAKE_PSTR: &str = "BitTorrent protocol";
pub const HANDSHAKE_PREFIX: [u8; 5] = [19, 66, 105, 116, 84];

pub const HAVE_LEN: usize = 5;
pub const REQUEST_LEN: usize = 13;
pub const PORT_LEN: usize = 3;

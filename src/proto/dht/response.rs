use std::collections::BTreeMap;

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Default, Clone)]
pub enum ResponseArgs {
    Pong {
        id: Vec<u8>,
    },
    FindNodeResp {
        id: Vec<u8>,
        nodes: Vec<u8>,
    },
    GetPeersWithValues {
        id: Vec<u8>,
        token: Option<Vec<u8>>,
        values: Vec<String>,
    },
    GetPeersWithNodes {
        id: Vec<u8>,
        token: Option<Vec<u8>>,
        nodes: Vec<u8>,
    },
    AnnouncePeerResp {
        id: Vec<u8>,
    },
    Other(BTreeMap<String, BencodeValue>),

    #[default]
    None,
}

impl ResponseArgs {
    pub fn parse(q: &str, r_map: Option<BTreeMap<String, BencodeValue>>) -> Self {
        let mut map = if let Some(d) = r_map {
            d
        } else {
            return Self::None;
        };
        match q {
            "ping" => Self::Pong {
                id: get_bytes(&mut map, "id"),
            },
            "find_node" => Self::FindNodeResp {
                id: get_bytes(&mut map, "id"),
                nodes: get_bytes(&mut map, "nodes"),
            },
            "get_peers" => {
                if map.contains_key("nodes") {
                    Self::GetPeersWithNodes {
                        id: get_bytes(&mut map, "id"),
                        token: {
                            let token = get_bytes(&mut map, "token");
                            if token.is_empty() {
                                None
                            } else {
                                Some(token)
                            }
                        },
                        nodes: get_bytes(&mut map, "nodes"),
                    }
                } else if map.contains_key("values") {
                    Self::GetPeersWithValues {
                        id: get_bytes(&mut map, "id"),
                        token: {
                            let token = get_bytes(&mut map, "token");
                            if token.is_empty() {
                                None
                            } else {
                                Some(token)
                            }
                        },
                        values: {
                            match map.remove("values") {
                                Some(BencodeValue::List(l)) => l
                                    .into_iter()
                                    .map(|v| match v {
                                        BencodeValue::Bytes(b) => String::from_utf8(b).ok(),
                                        _ => None,
                                    })
                                    .filter(|v| v.is_some())
                                    .map(|v| v.unwrap())
                                    .collect(),
                                _ => Vec::default(),
                            }
                        },
                    }
                } else {
                    Self::Other(map)
                }
            }
            "announce_peer" => Self::AnnouncePeerResp {
                id: get_bytes(&mut map, "id"),
            },
            _ => Self::Other(map),
        }
    }
}

#[inline]
fn get_bytes(map: &mut BTreeMap<String, BencodeValue>, key: &str) -> Vec<u8> {
    match map.remove(key) {
        Some(BencodeValue::Bytes(b)) => b,
        _ => Vec::default(),
    }
}

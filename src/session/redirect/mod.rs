use std::{collections::BTreeMap, net::SocketAddr};

use tokio::sync::{mpsc::Sender, Mutex};

use crate::proto::dht::DhtTransactionID;

type DhtRedirects = Mutex<BTreeMap<SocketAddr, BTreeMap<DhtTransactionID, Sender<Vec<u8>>>>>;

pub struct DhtRedirect {
    redirects: DhtRedirects,
}

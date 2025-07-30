use crate::proto::{
    bep15::{Bep15ConnectionID, Bep15Response, Bep15TransactionID},
    dht::{DhtTransactionID, KrpcMessage},
};
use std::{
    collections::BTreeMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};
use tokio::sync::mpsc::Sender;

type DhtRedirectMap = BTreeMap<SocketAddr, BTreeMap<DhtTransactionID, Sender<KrpcMessage>>>;

pub struct DhtRedirect(DhtRedirectMap);

impl Deref for DhtRedirect {
    type Target = DhtRedirectMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DhtRedirect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DhtRedirect {
    fn new() -> Self {
        Self(DhtRedirectMap::new())
    }
}

type Bep15RedirectMap =
    BTreeMap<SocketAddr, BTreeMap<(Bep15ConnectionID, Bep15TransactionID), Sender<Bep15Response>>>;

pub struct Bep15Redirect(Bep15RedirectMap);

impl Deref for Bep15Redirect {
    type Target = Bep15RedirectMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bep15Redirect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Bep15Redirect {
    fn new() -> Self {
        Self(Bep15RedirectMap::new())
    }
}

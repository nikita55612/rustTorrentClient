use crate::proto::{
    bep15::{Bep15ConnectionID, Bep15Response, Bep15TransactionID},
    dht::{DhtTransactionID, KrpcMessage},
};
use std::{
    collections::BTreeMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};
use tokio::sync::{mpsc::Sender as MpscSender, oneshot::Sender as OneshotSender};

pub enum RedirectChan<M> {
    Mpsc(MpscSender<M>),
    Oneshot(OneshotSender<M>),
}

#[inline]
async fn do_redirect<K: std::cmp::Ord, M>(
    map: &mut BTreeMap<SocketAddr, BTreeMap<K, RedirectChan<M>>>,
    addr: &SocketAddr,
    key: &K,
    msg: M,
) {
    if let Some(r) = map.get_mut(addr) {
        if let Some(rc) = r.get(key) {
            match rc {
                RedirectChan::<M>::Mpsc(s) => {
                    if s.is_closed() {
                        r.remove(key);
                    } else {
                        let _ = s.send(msg).await;
                    }
                }
                RedirectChan::<M>::Oneshot(_) => {
                    r.remove(key).map(|orc| {
                        if let RedirectChan::<M>::Oneshot(s) = orc {
                            let _ = s.send(msg);
                        }
                    });
                }
            }
        }
    }
}

type DhtRedirectMap = BTreeMap<SocketAddr, BTreeMap<DhtTransactionID, RedirectChan<KrpcMessage>>>;

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

    async fn do_redirect(&mut self, addr: &SocketAddr, key: &DhtTransactionID, msg: KrpcMessage) {
        do_redirect(self, addr, key, msg).await;
    }
}

type Bep15RedirectMap = BTreeMap<
    SocketAddr,
    BTreeMap<(Bep15ConnectionID, Bep15TransactionID), RedirectChan<Bep15Response>>,
>;

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

    async fn do_redirect(
        &mut self,
        addr: &SocketAddr,
        key: &(Bep15ConnectionID, Bep15TransactionID),
        msg: Bep15Response,
    ) {
        do_redirect(self, addr, key, msg).await;
    }
}

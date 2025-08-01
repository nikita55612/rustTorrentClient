use crate::proto::{
    bep15::{Bep15Response, Bep15TransactionID},
    dht::{DhtTransactionID, KrpcMessage},
};
use std::{
    collections::BTreeMap,
    net::SocketAddr,
    ops::{Deref, DerefMut},
};
use tokio::sync::{mpsc::Sender as MpscSender, oneshot::Sender as OneshotSender};

#[derive(Debug)]
pub enum RedirectChan<M> {
    Mpsc(MpscSender<M>),
    Oneshot(OneshotSender<M>),
}

pub type DhtResponseRouter = ResponseRouter<DhtTransactionID, KrpcMessage>;
pub type Bep15ResponseRouter = ResponseRouter<Bep15TransactionID, Bep15Response>;

#[derive(Debug)]
pub struct ResponseRouter<K, M>(BTreeMap<SocketAddr, BTreeMap<K, RedirectChan<M>>>)
where
    K: Ord;

impl<K: Ord, M> Deref for ResponseRouter<K, M> {
    type Target = BTreeMap<SocketAddr, BTreeMap<K, RedirectChan<M>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Ord, M> DerefMut for ResponseRouter<K, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Ord, M> ResponseRouter<K, M> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub async fn do_redirect(&mut self, addr: &SocketAddr, key: &K, msg: M) -> bool {
        if let Some(inner_map) = self.get_mut(addr) {
            if let Some(chan) = inner_map.get(key) {
                match chan {
                    RedirectChan::Mpsc(sender) => {
                        if sender.is_closed() {
                            inner_map.remove(key);
                            if inner_map.is_empty() {
                                self.remove(addr);
                            }
                        } else {
                            let _ = sender.send(msg).await;
                        }
                        return true;
                    }
                    RedirectChan::Oneshot(_) => {
                        if let Some(RedirectChan::Oneshot(sender)) = inner_map.remove(key) {
                            let _ = sender.send(msg);
                        }
                        if inner_map.is_empty() {
                            self.remove(addr);
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn insert_redirect(
        &mut self,
        addr: SocketAddr,
        transaction_id: K,
        chan: RedirectChan<M>,
    ) -> bool {
        self.0
            .entry(addr)
            .or_insert_with(BTreeMap::new)
            .insert(transaction_id, chan)
            .is_none()
    }

    pub fn remove_redirect(&mut self, addr: &SocketAddr, transaction_id: &K) -> bool {
        if let Some(map) = self.0.get_mut(addr) {
            let res = map.remove(transaction_id).is_some();
            if map.is_empty() {
                self.0.remove(addr);
            }
            res
        } else {
            false
        }
    }
}

use tokio::sync::watch::{self, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct Ctx<D: Clone + Send + Sync> {
    data: D,
    cancel_tx: Sender<bool>,
}

impl<D: Clone + Send + Sync> Ctx<D> {
    pub fn new(data: D) -> Self {
        let (cancel_tx, _) = watch::channel(false);
        Self { data, cancel_tx }
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn clone_data(&self) -> D {
        self.data.clone()
    }

    pub fn done(&self) -> Receiver<bool> {
        self.cancel_tx.subscribe()
    }

    pub fn cancel(&self) {
        let _ = self.cancel_tx.send(true);
    }

    pub fn is_cancelled(&self) -> bool {
        *self.cancel_tx.borrow()
    }
}

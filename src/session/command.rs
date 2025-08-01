use crate::session::state::SessionState;
use std::sync::Arc;
use tokio::{
    sync::mpsc::{self, Sender},
    task::JoinHandle,
};

pub enum SessionCommand {}

pub async fn spawn_command_handler(
    state: Arc<SessionState>,
) -> (Sender<SessionCommand>, JoinHandle<()>) {
    let (tx, mut rx) = mpsc::channel(4);

    let jh = tokio::spawn(async move {
        while let Some(command) = rx.recv().await {
            match command {}
        }
    });

    (tx, jh)
}

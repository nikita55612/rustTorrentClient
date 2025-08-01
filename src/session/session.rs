use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    error::{Error, Result},
    session::{
        background::{
            spawn_command_handler, spawn_tcp_incoming_listener, spawn_udp_listener, SessionCommand,
        },
        state::SessionState,
        SessionAlert,
    },
};

pub struct Session {
    cmd_tx: Sender<SessionCommand>,
    alert_rx: Receiver<SessionAlert>,
}

impl Session {
    pub async fn start() -> Result<Self> {
        let (cmd_tx, alert_rx) = spawn_new_session().await?;
        Ok(Self { cmd_tx, alert_rx })
    }

    #[inline]
    pub async fn send(&self, command: SessionCommand) -> Result<()> {
        self.cmd_tx
            .send(command)
            .await
            .map_err(|err| Error::SendSessionCommand(err.0))
    }

    #[inline]
    pub async fn recv(&mut self) -> Option<SessionAlert> {
        self.alert_rx.recv().await
    }
}

pub async fn spawn_new_session() -> Result<(Sender<SessionCommand>, Receiver<SessionAlert>)> {
    let (state, alert_rx) = SessionState::init().await?;
    let state = Arc::new(state);

    let udp_listener_handle = spawn_udp_listener(state.clone()).await;
    let tcp_incoming_listener_handle = spawn_tcp_incoming_listener(state.clone()).await;

    let (cmd_tx, command_jh) = spawn_command_handler(state.clone()).await;

    tokio::spawn(async move {
        let _ = command_jh.await;
        udp_listener_handle.abort();
        tcp_incoming_listener_handle.abort();
    });

    Ok((cmd_tx, alert_rx))
}

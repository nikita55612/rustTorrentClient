use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    error::{Error, Result},
    session::{
        background::{spawn_tcp_incoming_listener, spawn_udp_listener},
        spawn_command_handler,
        state::SessionState,
        SessionAlert, SessionCommand,
    },
};

pub struct Session {
    command_tx: Sender<SessionCommand>,
    alert_rx: Receiver<SessionAlert>,
}

impl Session {
    pub async fn start() -> Result<Self> {
        let (command_tx, alert_rx) = spawn_new_session().await?;
        Ok(Self {
            command_tx,
            alert_rx,
        })
    }

    #[inline]
    pub async fn send(&self, command: SessionCommand) -> Result<()> {
        self.command_tx
            .send(command)
            .await
            .map_err(|_| Error::SendSessionCommand)
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

    let (command_tx, command_jh) = spawn_command_handler(state.clone()).await;

    tokio::spawn(async move {
        let _ = command_jh.await;
        udp_listener_handle.abort();
        tcp_incoming_listener_handle.abort();
    });

    Ok((command_tx, alert_rx))
}

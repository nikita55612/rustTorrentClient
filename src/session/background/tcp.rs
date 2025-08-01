use crate::session::state::SessionState;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub async fn spawn_tcp_incoming_listener(state: Arc<SessionState>) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            match state.tcp_listener.accept().await {
                Ok((socket, _addr)) => {
                    // process_socket
                }
                Err(_err) => {
                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                }
            }
        }
    })
}

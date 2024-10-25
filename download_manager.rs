use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::Error;
use crate::message::{Message, MessageID};
use crate::PeerConnection;
use crate::piece::{PieceManager, Block};

pub struct DownloadManager {
    piece_manager: Arc<Mutex<PieceManager>>,
    peers: Vec<PeerConnection>,
}

impl DownloadManager {
    pub fn new(piece_manager: PieceManager, peers: Vec<PeerConnection>) -> Self {
        Self {
            piece_manager: Arc::new(Mutex::new(piece_manager)),
            peers,
        }
    }

    pub async fn start_download(&mut self) -> Result<(), Error> {
        let mut download_tasks = Vec::new();

        for mut peer in self.peers.drain(..) {
            let piece_manager = Arc::clone(&self.piece_manager);
            
            let task = tokio::spawn(async move {
                loop {
                    // Request next block
                    if let Ok(mut piece_manager) = piece_manager.lock().await {
                        if piece_manager.is_complete() {
                            break;
                        }

                        // Получаем следующий блок для запроса
                        if let Some(block) = piece_manager.next_request(&peer_bitfield) {
                            let request = Message::new(
                                MessageID::Request,
                                vec![
                                    &(block.index as u32).to_be_bytes()[..],
                                    &(block.begin as u32).to_be_bytes()[..],
                                    &(block.length as u32).to_be_bytes()[..],
                                ].concat(),
                            );

                            if let Err(e) = peer.send(request).await {
                                eprintln!("Failed to send request: {}", e);
                                break;
                            }

                            // Получаем ответ
                            match peer.receive().await {
                                Ok(response) => {
                                    if let MessageID::Piece = response.id {
                                        let piece_index = u32::from_be_bytes(response.payload[0..4].try_into().unwrap()) as usize;
                                        let begin = u32::from_be_bytes(response.payload[4..8].try_into().unwrap()) as usize;
                                        let block_data = response.payload[8..].to_vec();

                                        if let Ok(true) = piece_manager.add_piece_block(piece_index, begin, block_data).await {
                                            println!("Downloaded piece {}, Progress: {:.2}%", 
                                                piece_index, 
                                                piece_manager.progress() * 100.0
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Failed to receive piece: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                }
            });

            download_tasks.push(task);
        }

        // Ждем завершения всех задач загрузки
        for task in download_tasks {
            if let Err(e) = task.await {
                eprintln!("Download task failed: {}", e);
            }
        }

        Ok(())
    }
}
use std::collections::HashMap;
use crate::error::Error;
use crate::Bitfield;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

const BLOCK_SIZE: usize = 16384; // 16KB

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub index: usize,
    pub begin: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Piece {
    pub index: usize,
    pub length: usize,
    pub blocks: Vec<Block>,
    pub downloaded_blocks: HashMap<(usize, usize), Vec<u8>>,
}

impl Piece {
    pub fn new(index: usize, piece_length: usize) -> Self {
        let mut blocks = Vec::new();
        let mut begin = 0;
        
        while begin < piece_length {
            let length = if piece_length - begin < BLOCK_SIZE {
                piece_length - begin
            } else {
                BLOCK_SIZE
            };
            
            blocks.push(Block {
                index,
                begin,
                length,
            });
            
            begin += BLOCK_SIZE;
        }

        Self {
            index,
            length: piece_length,
            blocks,
            downloaded_blocks: HashMap::new(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.downloaded_blocks.len() == self.blocks.len()
    }

    pub fn add_block(&mut self, begin: usize, data: Vec<u8>) {
        self.downloaded_blocks.insert((self.index, begin), data);
    }

    pub fn get_data(&self) -> Option<Vec<u8>> {
        if !self.is_complete() {
            return None;
        }

        let mut data = Vec::with_capacity(self.length);
        let mut begin = 0;

        while begin < self.length {
            if let Some(block_data) = self.downloaded_blocks.get(&(self.index, begin)) {
                data.extend(block_data);
            }
            begin += BLOCK_SIZE;
        }

        Some(data)
    }
}

#[derive(Debug)]
pub struct PieceManager {
    pieces: Vec<Piece>,
    piece_length: usize,
    total_length: u64,
    downloaded_pieces: Bitfield,
    file: File,
}

impl PieceManager {
    pub async fn new(piece_length: usize, total_length: u64, pieces_count: usize, path: &str) -> Result<Self, Error> {
        let mut pieces = Vec::with_capacity(pieces_count);
        
        for i in 0..pieces_count {
            let piece_size = if i == pieces_count - 1 {
                total_length as usize % piece_length
            } else {
                piece_length
            };
            pieces.push(Piece::new(i, piece_size));
        }

        let downloaded_pieces = Bitfield::new(vec![0; (pieces_count + 7) / 8]);
        let file = File::create(path).await?;

        Ok(Self {
            pieces,
            piece_length,
            total_length,
            downloaded_pieces,
            file,
        })
    }

    pub fn next_request(&self, peer_bitfield: &Bitfield) -> Option<Block> {
        for piece in &self.pieces {
            if !self.downloaded_pieces.has_piece(piece.index) && peer_bitfield.has_piece(piece.index) {
                for block in &piece.blocks {
                    if !piece.downloaded_blocks.contains_key(&(block.index, block.begin)) {
                        return Some(*block);
                    }
                }
            }
        }
        None
    }

    pub async fn add_piece_block(&mut self, piece_index: usize, begin: usize, data: Vec<u8>) -> Result<bool, Error> {
        if let Some(piece) = self.pieces.get_mut(piece_index) {
            piece.add_block(begin, data);

            if piece.is_complete() {
                if let Some(piece_data) = piece.get_data() {
                    let offset = piece_index as u64 * self.piece_length as u64;
                    //self.file.seek(std::io::SeekFrom::Start(offset)).await?;
                    self.file.write_all(&piece_data).await?;
                    self.downloaded_pieces.set_piece(piece_index);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn is_complete(&self) -> bool {
        self.downloaded_pieces.count_pieces() == self.pieces.len()
    }

    pub fn progress(&self) -> f64 {
        self.downloaded_pieces.count_pieces() as f64 / self.pieces.len() as f64
    }
}
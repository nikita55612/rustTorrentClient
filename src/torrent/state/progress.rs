#[derive(Debug, Default, Clone)]
pub struct TorrentProgress {
    pub num_pieces: usize,
    pub have_pieces: usize,
    pub uploaded: u64,
    pub downloaded: u64,
    pub left: u64,
}

impl TorrentProgress {
    pub fn download_percentage(&self) -> f64 {
        let total = self.downloaded + self.left;
        if total == 0 {
            0.
        } else {
            self.downloaded as f64 / total as f64
        }
    }

    pub fn have_pieces_percentage(&self) -> f64 {
        if self.num_pieces == 0 {
            0.
        } else {
            self.have_pieces as f64 / self.num_pieces as f64
        }
    }

    pub fn remaining_pieces(&self) -> usize {
        self.num_pieces.saturating_sub(self.have_pieces)
    }

    pub fn is_complete(&self) -> bool {
        self.have_pieces >= self.num_pieces
    }
}

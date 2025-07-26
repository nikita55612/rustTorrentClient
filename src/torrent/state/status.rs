#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum TorrentStatus {
    #[default]
    Waiting,
    Started,
    Downloading,
    Seeding,
    Stopped,
    Completed,
    Error(String),
}

impl Into<String> for TorrentStatus {
    fn into(self) -> String {
        self.as_str().into()
    }
}

impl TorrentStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "waiting" => TorrentStatus::Waiting,
            "started" => TorrentStatus::Started,
            "downloading" => TorrentStatus::Downloading,
            "seeding" => TorrentStatus::Seeding,
            "stopped" => TorrentStatus::Stopped,
            "completed" => TorrentStatus::Completed,
            "error" => TorrentStatus::Error("".into()),
            _ => TorrentStatus::Waiting,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            TorrentStatus::Waiting => "waiting",
            TorrentStatus::Started => "started",
            TorrentStatus::Downloading => "downloading",
            TorrentStatus::Seeding => "seeding",
            TorrentStatus::Stopped => "stopped",
            TorrentStatus::Completed => "completed",
            TorrentStatus::Error(_) => "error",
        }
    }
}

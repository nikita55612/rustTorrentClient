#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Event {
    #[default]
    None,
    Completed,
    Started,
    Stopped,
}

impl Event {
    pub fn from_str(s: &str) -> Self {
        match s {
            "started" => Self::Started,
            "completed" => Self::Completed,
            "stopped" => Self::Stopped,
            _ => Self::None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "",
            Self::Completed => "completed",
            Self::Started => "started",
            Self::Stopped => "stopped",
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::Completed => 1,
            Self::Started => 2,
            Self::Stopped => 3,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Event {
    #[default]
    Started,
    Completed,
    Stopped,
    Empty,
}

impl Into<String> for Event {
    fn into(self) -> String {
        self.as_str().into()
    }
}

impl Event {
    pub fn from_str(s: &str) -> Self {
        match s {
            "started" => Event::Started,
            "completed" => Event::Completed,
            "stopped" => Event::Stopped,
            _ => Event::Empty,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Started => "started",
            Self::Completed => "completed",
            Self::Stopped => "stopped",
            Self::Empty => "",
        }
    }
}

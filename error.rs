use std::fmt;

#[derive(Debug)]
pub enum Error {
    FromUtf8Error(std::string::FromUtf8Error),
    TryFromSliceError(std::array::TryFromSliceError),
    IoError(std::io::Error),
    PeerConnectionError(String),
    TimeoutError(tokio::time::error::Elapsed),
    PeerShutdownConnectionError,
    ReqwestError(reqwest::Error),
    ParseTorrentError,
    ParseInfoHashError,
    SerdeBencodeError(serde_bencode::Error),
    AnyError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FromUtf8Error(e) => write!(f, "UTF-8 Error: {}", e),
            Self::TryFromSliceError(e) => write!(f, "Slice Conversion Error: {}", e),
            Self::IoError(e) => write!(f, "IoError: {}", e),
            Self::PeerConnectionError(e) => write!(f, "PeerConnectionError: {}", e),
            Self::TimeoutError(e) => write!(f, "TimeoutError: {}", e),
            Self::PeerShutdownConnectionError => write!(f, "PeerShutdownConnectionError"),
            Self::ReqwestError(e) => write!(f, "ReqwestError: {}", e),
            Self::ParseTorrentError => write!(f, "ParseTorrentError"),
            Self::ParseInfoHashError => write!(f, "ParseInfoHashError"),
            Self::SerdeBencodeError(e) => write!(f, "SerdeBencodeError: {}", e),
            Self::AnyError(e) => write!(f, "Other Error: {}", e),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(err)
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error::TryFromSliceError(err)
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(err: tokio::time::error::Elapsed) -> Self {
        Error::TimeoutError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<serde_bencode::Error> for Error {
    fn from(err: serde_bencode::Error) -> Self {
        Error::SerdeBencodeError(err)
    }
}



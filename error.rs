use std::fmt;

#[derive(Debug)]
pub enum Error {
    FromUtf8Error(std::string::FromUtf8Error),
    TryFromSliceError(std::array::TryFromSliceError),
    IoError(std::io::Error),
    PeerConnectionError(String),
    TimeoutError(tokio::time::error::Elapsed),
    PeerShutdownConnectionError,
    AnyError(String),
}



impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FromUtf8Error(e) => write!(f, "UTF-8 Error: {}", e),
            Error::TryFromSliceError(e) => write!(f, "Slice Conversion Error: {}", e),
            Error::IoError(e) => write!(f, "IoError: {}", e),
            Error::PeerConnectionError(e) => write!(f, "PeerConnectionError: {}", e),
            Error::TimeoutError(e) => write!(f, "TimeoutError: {}", e),
            Error::PeerShutdownConnectionError => write!(f, "PeerShutdownConnectionError"),
            Error::AnyError(e) => write!(f, "Other Error: {}", e),
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



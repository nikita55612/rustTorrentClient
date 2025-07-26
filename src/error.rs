use reqwest::Error as ReqwestError;
use serde_bencode::Error as SerDeBencodeError;
use std::io::Error as StdIoError;
use std::result::Result as StdResult;
use thiserror::Error as ThisError;
use tokio::time::error::Elapsed;

pub type Result<T> = StdResult<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("StdIoError: {0:?}")]
    StdIo(#[from] StdIoError),

    #[error("SerDeBencodeError: {0:?}")]
    SerDeBencode(#[from] SerDeBencodeError),

    #[error("TryFromSliceError: {0:?}")]
    TryFromSlice(#[from] std::array::TryFromSliceError),

    #[error("FromHexError: {0:?}")]
    FromHex(#[from] hex::FromHexError),

    #[error("ReqwestError: {0:?}")]
    Reqwest(#[from] ReqwestError),

    #[error("{0:?}")]
    Elapsed(#[from] Elapsed),

    #[error("AnnounceBuilderError: {0:?}")]
    AnnounceBuilder(String),

    #[error("TrackerFailureReason: {0:?}")]
    TrackerFailureReason(String),

    #[error("ParseMagnetLinkError: {0:?}")]
    ParseMagnetLink(String),

    #[error("InvalidBep15AnnounceResponse: {0:?}")]
    InvalidBep15AnnounceResponse(String),

    #[error("InvalidBep15ConnectResponse: {0:?}")]
    InvalidBep15ConnectResponse(String),

    #[error("{0}")]
    Custom(String),
}

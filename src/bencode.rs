use crate::error::Result;
use crate::hash::Hash;
use serde::Deserialize;
use serde_bencode::Error as SerDeBencodeError;
use std::ops::Deref;

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

impl Torrent {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_bencode::from_bytes::<Self>(bytes)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,

    #[serde(rename = "piece length")]
    pub piece_length: usize,

    #[serde(default, with = "serde_bytes")]
    pub pieces: Vec<u8>,

    #[serde(default)]
    length: Option<usize>,

    #[serde(default)]
    pub files: Option<Vec<File>>,
}

impl Info {
    pub fn length(&self) -> usize {
        if let Some(length) = self.length {
            length
        } else if let Some(files) = &self.files {
            files.iter().map(|f| f.length).sum()
        } else {
            0
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoHash(Hash);

impl Deref for InfoHash {
    type Target = Hash;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl InfoHash {
    pub fn new(s: [u8; 20]) -> Self {
        Self(Hash::new(s))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let value = serde_bencode::de::from_bytes::<BencodeValue>(bytes)?;
        let mut info_bytes = Vec::new();
        if let BencodeValue::Dict(d) = value {
            let info = d
                .get("info".as_bytes())
                .ok_or(SerDeBencodeError::MissingField("info".into()))?;
            info_bytes = serde_bencode::ser::to_bytes(info)?;
        }

        if info_bytes.is_empty() {
            return Err(SerDeBencodeError::InvalidValue("Not Dict".into()).into());
        }

        Ok(Self(Hash::from_bytes(&info_bytes)))
    }
}

#[derive(Debug, Deserialize)]
pub struct TrackerResponse {
    pub interval: usize,

    #[serde(with = "serde_bytes")]
    pub peers: Vec<u8>,
}

impl TrackerResponse {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_bencode::de::from_bytes::<Self>(bytes)?)
    }
}

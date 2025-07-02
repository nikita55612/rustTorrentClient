/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
use crate::metadata::{fileinfo::FileInfo, filetree::FileTree};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    #[serde(rename = "piece length")]
    pub piece_length: u64,

    #[serde(default, with = "serde_bytes")]
    pub pieces: Vec<u8>,

    pub name: String,

    #[serde(default)]
    pub length: Option<u64>,

    #[serde(default)]
    pub files: Vec<FileInfo>,

    #[serde(default, rename = "meta version")]
    pub meta_version: Option<i64>,

    #[serde(default, rename = "file tree")]
    pub file_tree: Option<FileTree>,
}

impl Info {
    pub fn total_length(&self) -> u64 {
        0
    }
}

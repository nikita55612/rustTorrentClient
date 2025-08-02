/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
use super::file::{FileInfo, FileTree};
use crate::error::Result;
use serde::Deserialize;

type BencodeValue = serde_bencode::value::Value;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Info {
    #[serde(default)]
    pub name: String,

    #[serde(rename = "piece length")]
    pub piece_length: u64,

    #[serde(default, with = "serde_bytes")]
    pieces: Option<Vec<u8>>,

    #[serde(default)]
    length: Option<u64>,

    #[serde(default)]
    files: Option<Vec<FileInfo>>,

    #[serde(default, rename = "meta version")]
    meta_version: Option<i32>,

    // file_tree_value -> file_tree
    #[serde(default, rename = "file tree")]
    file_tree_value: Option<BencodeValue>,

    #[serde(skip)]
    file_tree: Option<FileTree>,
}

impl Info {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut info = serde_bencode::from_bytes::<Self>(bytes)?;

        if let Some(value) = info.file_tree_value.take() {
            info.file_tree.replace(FileTree::from_bencode_value(value)?);
        }

        Ok(info)
    }

    #[inline]
    pub fn is_single_file_mode(&self) -> bool {
        self.length.is_some()
    }

    #[inline]
    pub fn meta_version(&self) -> i32 {
        self.meta_version.unwrap_or(1)
    }

    #[inline]
    pub fn total_length(&self) -> u64 {
        if let Some(length) = self.length {
            return length;
        }
        if let Some(ref files) = self.files {
            return files.iter().map(|f| f.length).sum();
        }
        if let Some(ref file_tree) = self.file_tree {
            return file_tree.total_length();
        }
        0
    }

    #[inline]
    pub fn num_pieces(&self) -> usize {
        if self.meta_version() == 2 {
            let total = self.total_length();
            return ((total + self.piece_length - 1) / self.piece_length) as usize;
        }
        if let Some(pieces) = &self.pieces {
            return pieces.len() / 20;
        }
        0
    }
}

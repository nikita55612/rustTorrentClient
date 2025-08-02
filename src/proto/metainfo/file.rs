/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
use crate::error::Result;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

type BencodeValue = serde_bencode::value::Value;
type BencodeError = serde_bencode::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct FileInfo {
    pub length: u64,
    pub path: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum FileTree {
    Dir(BTreeMap<String, FileTree>),
    File { length: u64, pieces_root: Vec<u8> },
}

impl FileTree {
    pub fn from_bencode_value(value: BencodeValue) -> Result<FileTree> {
        Ok(de_file_tree::bencode_to_file_tree(value).map_err(|e| BencodeError::Custom(e))?)
    }

    #[inline]
    pub fn total_length(&self) -> u64 {
        match self {
            FileTree::File { length, .. } => *length,
            FileTree::Dir(entries) => entries.values().map(|e| e.total_length()).sum(),
        }
    }

    pub fn iter_files(&self) -> Vec<(PathBuf, &FileTree)> {
        let mut files = Vec::new();
        self.collect_files(Path::new(""), &mut files);
        files
    }

    #[inline]
    fn collect_files<'a>(&'a self, base: &Path, acc: &mut Vec<(PathBuf, &'a FileTree)>) {
        match self {
            FileTree::File { .. } => {
                acc.push((base.to_path_buf(), self));
            }
            FileTree::Dir(map) => {
                for (name, subtree) in map {
                    let path = base.join(name);
                    subtree.collect_files(&path, acc);
                }
            }
        }
    }
}

// type ExtendedFileAttrs struct {
// 	Attr        string   `bencode:"attr,omitempty"`
// 	SymlinkPath []string `bencode:"symlink path,omitempty"`
// 	Sha1        string   `bencode:"sha1,omitempty"`
// }

mod de_file_tree {
    use super::*;
    use std::result::Result;

    pub fn bencode_to_file_tree(value: BencodeValue) -> Result<FileTree, String> {
        let BencodeValue::Dict(dict) = value else {
            return Err("Expected dictionary at root level".into());
        };

        de_directory(dict)
    }

    #[inline]
    fn de_directory(
        dict: std::collections::HashMap<Vec<u8>, BencodeValue>,
    ) -> Result<FileTree, String> {
        let mut dir_map = BTreeMap::new();

        for (key, value) in dict {
            if key.is_empty() {
                return Err("Root directory cannot be a file".into());
            }

            let path = sanitize_path(key);
            let subtree = de_entry(value)?;
            dir_map.insert(path, subtree);
        }

        Ok(FileTree::Dir(dir_map))
    }

    #[inline]
    fn de_entry(value: BencodeValue) -> Result<FileTree, String> {
        let BencodeValue::Dict(dict) = value else {
            return Err("Expected dictionary in file tree entry".into());
        };

        if let Some(file_info) = dict.get(&[] as &[u8]) {
            return de_file(file_info);
        }

        de_directory(dict)
    }

    #[inline]
    fn de_file(value: &BencodeValue) -> Result<FileTree, String> {
        let BencodeValue::Dict(dict) = value else {
            return Err("Expected dictionary for file info".into());
        };

        let length = dict
            .get(b"length".as_slice())
            .and_then(|v| match v {
                BencodeValue::Int(l) if *l >= 0 => Some(*l as u64),
                _ => None,
            })
            .ok_or("Missing or invalid 'length' field")?;

        let pieces_root = dict
            .get(b"pieces root".as_slice())
            .and_then(|v| match v {
                BencodeValue::Bytes(bytes) => Some(bytes.clone()),
                _ => None,
            })
            .unwrap_or_default();

        Ok(FileTree::File {
            length,
            pieces_root,
        })
    }

    fn sanitize_path(raw: Vec<u8>) -> String {
        if raw == b"." || raw == b".." {
            String::from("_")
        } else {
            String::from_utf8(raw).unwrap_or("_".into())
        }
    }
}

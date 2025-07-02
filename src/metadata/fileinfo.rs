/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FileInfo {
    pub length: u64,
    pub path: Vec<String>,
}

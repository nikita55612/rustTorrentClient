/// https://wiki.theory.org/BitTorrentSpecification#Metainfo_File_Structure
pub mod file;
mod info;
mod metainfo;

pub use info::Info;
pub use metainfo::{AnnounceList, MetaInfo};

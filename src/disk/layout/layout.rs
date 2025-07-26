use std::{ffi::OsStr, path::Path, sync::LazyLock};

use super::{FilesLayout, SingleFileLayout, TreeLayout};
use crate::{error::Result, torrent::Info};

const SEP: LazyLock<&OsStr> = LazyLock::new(|| OsStr::new("/"));

#[derive(Debug, Clone)]
pub enum Layout {
    SingleFile(SingleFileLayout),
    Files(FilesLayout),
    Tree(TreeLayout),
}

impl Layout {
    pub fn from_torrent_info(info: &Info) -> Self {
        if info.is_single_file_mode() {
            let file_layout = SingleFileLayout {
                name: info.name.clone().into(),
                length: info.total_length(),
            };
            return Self::SingleFile(file_layout);
        }
        if let Some(ref files) = info.files {
            let mut files_layout = FilesLayout::new(&info.name);
            for file in files {
                files_layout.add_file(&file.path.join(*SEP), file.length);
            }
            return Self::Files(files_layout);
        }
        if let Some(ref _tree) = info.file_tree {
            todo!()
        }
        return Self::SingleFile(SingleFileLayout {
            name: info.name.clone().into(),
            length: 0,
        });
    }

    pub async fn init<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        match self {
            Self::SingleFile(file) => {
                let file_path = path.as_ref().join(&file.name);
                tokio::fs::File::create(file_path)
                    .await?
                    .set_len(file.length)
                    .await?;
            }
            Self::Files(files) => {
                let root_path = path.as_ref().join(&files.root_dir);
                for file in files.files.iter() {
                    let file_path = root_path.join(&file.path);
                    tokio::fs::File::create(file_path)
                        .await?
                        .set_len(file.length)
                        .await?;
                }
            }
            Self::Tree(_tree) => {
                todo!()
            }
        }
        Ok(())
    }
}

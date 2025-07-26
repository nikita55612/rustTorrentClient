use std::{
    collections::HashSet,
    ffi::OsString,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct File {
    pub path: PathBuf,
    pub length: u64,
}

#[derive(Debug, Clone)]
pub struct FilesLayout {
    pub root_dir: OsString,
    pub files: HashSet<File>,
    pub length: u64,
}

impl FilesLayout {
    pub fn new<D: Into<OsString>>(root_dir: D) -> Self {
        Self {
            root_dir: root_dir.into(),
            files: HashSet::new(),
            length: 0,
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P, length: u64) {
        let file = File {
            path: path.as_ref().to_owned(),
            length,
        };
        if self.files.insert(file) {
            self.length += length;
        }
    }
}

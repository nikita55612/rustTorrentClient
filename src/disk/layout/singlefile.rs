use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct SingleFileLayout {
    pub name: OsString,
    pub length: u64,
}

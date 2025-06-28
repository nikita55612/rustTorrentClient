use std::io::SeekFrom;

use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncSeekExt, AsyncWriteExt},
};

use crate::error::Result;

// disk layout

pub async fn create_file(path: &str, len: usize) -> Result<()> {
    File::create(path).await?.set_len(len as u64).await?;

    Ok(())
}

pub async fn write_chunk(path: &str, offset: u64, data: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new().write(true).open(path).await?;

    file.seek(SeekFrom::Start(offset)).await?;
    file.write_all(data).await?;

    Ok(())
}

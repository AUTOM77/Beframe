use std::path::PathBuf;

use md5::{Md5, Digest};
use image::RgbImage;
use ffmpeg_next::{codec, format, frame, media, software};

use tokio::fs;
use tokio::io::AsyncWriteExt;

static _CACHE: &str = "fc";

#[derive(Debug)]
pub struct X264Video {
    path: PathBuf,
    local: String
}

impl X264Video {
    pub fn load(path: PathBuf) -> Self {
        let buffer = std::fs::read(&path).unwrap();
        let mut hasher = Md5::new();
        hasher.update(&buffer);
        let digest = hasher.finalize();
        let local = format!("{}/{:x}", _CACHE, digest);

        Self {
            path,
            local
        }
    }

    pub async fn mkdir(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.local).await?;
        Ok(())
    }

}
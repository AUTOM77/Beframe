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

    pub async fn processing(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.mkdir().await?;

        ffmpeg_next::init()?;

        let mut ictx = format::input(&self.path)?;

        let input = ictx
                .streams()
                .best(media::Type::Video)
                .ok_or(ffmpeg_next::Error::StreamNotFound)?;
        let idx = input.index();
        let ctx = codec::context::Context::from_parameters(input.parameters())?;

        let mut decoder = ctx.decoder().video()?;

        let mut count = 0;
        let mut valid = 0;

        for (stream, packet) in ictx.packets() {
            if stream.index() == idx {
                decoder.send_packet(&packet)?;
                let mut decoded_frame = frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    if count % 5 ==0 {
                        // let _ = self.add_frame(&decoded_frame, valid)?;
                        valid+=1;
                    }
                    count+=1;
                }
            }
        }
        decoder.send_eof()?;
        Ok(())
    }

    pub async fn mkdir(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.local).await?;
        Ok(())
    }

}
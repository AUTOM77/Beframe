use std::fs;
use std::path::PathBuf;

use md5::{Md5, Digest};
use image::RgbImage;
use ffmpeg_next::{codec, format, frame, media, software};

static _CACHE: &str = "fc";

#[derive(Debug)]
pub struct X264Video {
    path: PathBuf,
    local: String
}

impl X264Video {
    pub fn load(path: PathBuf) -> Self {
        let buffer = fs::read(&path).unwrap();
        let mut hasher = Md5::new();
        hasher.update(&buffer);
        let digest = hasher.finalize();
        let local = format!("{}/{:x}", _CACHE, digest);

        X264Video {
            path,
            local
        }
    }

    pub fn processing(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.mkdir();
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
        for (stream, packet) in ictx.packets() {
            if stream.index() == idx {
                decoder.send_packet(&packet)?;
                let mut decoded_frame = frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    let _ = self.add_frame(&decoded_frame, count)?;
                    count+=1;
                }
            }
        }
        decoder.send_eof()?;
        Ok(())
    }

    pub fn add_frame(&self, _frame: &frame::video::Video, i: usize) -> Result<(), Box<dyn std::error::Error>> {
        let f = format!("{}/{:04}.png", self.local, i);

        let w = _frame.width();
        let h = _frame.height();
        let fmt = _frame.format();

        let mut scaler = software::scaling::context::Context::get(
            fmt,
            w, h,
            format::Pixel::RGB24,
            w, h,
            software::scaling::flag::Flags::BILINEAR,
        )?;

        let mut frame = frame::video::Video::empty();
        scaler.run(&_frame, &mut frame)?;

        let img = RgbImage::from_raw(w, h, frame.data(0).to_vec()).unwrap(); 
        img.save(f)?; 
        Ok(())
    }

    pub fn mkdir(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(&self.local)?;
        Ok(())
    }
}
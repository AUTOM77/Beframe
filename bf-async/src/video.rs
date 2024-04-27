use std::fs;
use md5::{Md5, Digest};
use image::RgbImage;
use ffmpeg_next::{codec, format, frame, media, software};

static _CACHE: &str = "fc";

#[derive(Debug)]
pub struct X264Video<'a> {
    path: &'a str,
    local: &'a str
}

impl<'a> X264Video<'a> {
    pub fn load(path: &'a str) -> Self {
        let buffer = fs::read(path).unwrap();
        let mut hasher = Md5::new();
        hasher.update(&buffer);
        let digest = hasher.finalize();

        let hash = format!("{:x}", digest).leak();
        let local = format!("{}/{}", _CACHE, &hash).leak();

        X264Video {
            path,
            local
        }
    }

    pub fn processing(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.mkdir();
        ffmpeg_next::init()?;

        let mut ictx = format::input(self.path)?;

        let input = ictx
                .streams()
                .best(media::Type::Video)
                .ok_or(ffmpeg_next::Error::StreamNotFound)?;
        let idx = input.index();

        let ctx = codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = ctx.decoder().video()?;

        let mut scaler = software::scaling::context::Context::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                format::Pixel::RGB24,
                decoder.width(),
                decoder.height(),
                software::scaling::flag::Flags::BILINEAR,
            )?;

        let mut count = 0;

        for (stream, packet) in ictx.packets() {
            if stream.index() == idx {
                decoder.send_packet(&packet)?;
                let mut decoded_frame = frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    let mut rgb_frame = frame::video::Video::empty();
                    scaler.run(&decoded_frame, &mut rgb_frame)?;
                    self.add_frame(&rgb_frame, count)?;
                    count+=1;
                }
            }
        }
        decoder.send_eof()?;
        Ok(())
    }

    pub fn add_frame(&self, frame: &frame::video::Video, i: u32) -> Result<(), Box<dyn std::error::Error>> {
        let f = format!("{}/{:04}.png", self.local, i);
        let width = frame.width();
        let height = frame.height();
        let data = frame.data(0).to_vec();
        
        let img = RgbImage::from_raw(width, height, data.to_vec()).unwrap(); 
        img.save(f)?; 
        Ok(())
    }

    pub fn mkdir(&self) -> Result<(), std::io::Error> {
        fs::create_dir_all(self.local)?;
        Ok(())
    }
}
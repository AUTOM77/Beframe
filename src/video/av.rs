use ffmpeg_next::{codec, format, frame, media, software};
use std::path::PathBuf;
use std::io::Write;
use md5::{Md5, Digest};

use image::RgbImage;

pub struct X264Video {
    path: PathBuf,
    local: PathBuf
}

impl X264Video {
    pub fn from(path: PathBuf, root: &PathBuf) -> Self {
        let _pth = path.clone();
        let fname = _pth.file_stem().and_then(|stem| stem.to_str()).unwrap();
        Self {
            path,
            local: root.join(fname),
        }
    }

    pub fn load(buffer: Vec<u8>, root: &PathBuf) ->  Result<X264Video, Box<dyn std::error::Error>> {
        let _acc:PathBuf = PathBuf::from("/dev/shm");

        let mut hasher = Md5::new();
        hasher.update(&buffer);
        let digest = hasher.finalize();

        let path = _acc.join(format!("{:x}.mp4", digest));
        let local = root.join(format!("{:x}", digest));

        let f = std::fs::File::create(&path)?;
        let mut w = std::io::BufWriter::new(f);
        let _ = w.write_all(&buffer)?;
        let _ = w.flush()?;

        Ok( Self{path,local} )
    }

    pub fn clip(&self) -> Result<(), Box<dyn std::error::Error>> {
        ffmpeg_next::init()?;
        let _ = self.mkdir();
        let mut ictx = format::input(&self.path)?;
        let input = ictx
                .streams()
                .best(media::Type::Video)
                .ok_or(ffmpeg_next::Error::StreamNotFound)?;

        let idx = input.index();

        let ctx = codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = ctx.decoder().video()?;

        let fmt = decoder.format();
        let w = decoder.width();
        let h = decoder.height();

        let mut scaler = software::scaling::context::Context::get(
                fmt, 
                w, h,
                format::Pixel::RGB24,
                w, h,
                software::scaling::flag::Flags::BILINEAR,
            )?;
        
        let mut i=0;
        let mut count=0;
        for (stream, packet) in ictx.packets() {
            if stream.index() == idx {
                decoder.send_packet(&packet)?;
                let mut decoded_frame = frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    if count % 9 == 0{
                        let mut rgb_frame = frame::video::Video::empty();
                        scaler.run(&decoded_frame, &mut rgb_frame)?;
                        let img = RgbImage::from_raw(w, h, rgb_frame.data(0).to_vec()).unwrap(); 
                        img.save(self.local.join(format!("{:04}.jpg", i)))?; 
                        i+=1;
                    }
                    count+=1;
                }
            }
        }

        decoder.send_eof()?;
        Ok(())
    }

    pub fn mkdir(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.local)?;
        Ok(())
    }

    pub fn drop(&self) -> Result<(), std::io::Error> {
        std::fs::remove_file(&self.path)?;
        Ok(())
    }
}
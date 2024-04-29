use md5::{Md5, Digest};
use ffmpeg_next::{codec, format, frame, media, software};

static _CACHE: &str = "fc";

pub struct X264Video {
    buffer: Vec<u8>,
    local: String
}

impl X264Video {
    pub fn load(buff: &Vec<u8>) -> Self {
        let mut hasher = Md5::new();
        hasher.update(buff);
        let digest = hasher.finalize();
        let local = format!("{}/{:x}", _CACHE, digest);

        let buffer = buff.to_vec();
        Self {
            buffer,
            local
        }
    }

    pub fn from(path: &std::path::PathBuf) -> Self {
        let buff = std::fs::read(path).unwrap();
        Self::load(&buff)
    }

    pub fn clip(&self) -> Result<(), Box<dyn std::error::Error>> {
            ffmpeg_next::init()?;
            let _ = self.mkdir();

            let mut ictx = {
                let io_ctx = format::io::IOContext::from_bytes(&self.buffer)?;
                format::input_from_io(io_ctx)?
            };

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
                        if count % 5 == 0{
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

    pub fn hash(&self) -> String {
        self.local.clone()
    }
}
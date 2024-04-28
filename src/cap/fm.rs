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

    pub fn mkdir(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.local)?;
        Ok(())
    }

    pub fn hash(&self) -> String {
        self.local.clone()
    }
}

pub struct Frame {
    buffer: Vec<u8>,
    width: u32,
    height: u32
}

pub struct Beframe {
    path: std::path::PathBuf,
    local: String
}

impl Beframe {
    pub fn from(path: std::path::PathBuf) -> Self {
        let buff = std::fs::read(&path).unwrap();
        let mut hasher = Md5::new();
        hasher.update(buff);
        let digest = hasher.finalize();
        let local = format!("{}/{:x}", _CACHE, digest);

        Self {
            path,
            local
        }
    }

    pub fn clip(&self) -> Result<Vec<Frame>, Box<dyn std::error::Error>> {
        let mut frames: Vec<Frame> = Vec::new();
        ffmpeg_next::init()?;

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

        for (stream, packet) in ictx.packets() {
            if stream.index() == idx {
                decoder.send_packet(&packet)?;
                let mut decoded_frame = frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    let mut rgb_frame = frame::video::Video::empty();
                    scaler.run(&decoded_frame, &mut rgb_frame)?;
                    frames.push(Frame{
                        buffer: rgb_frame.data(0).to_vec(),
                        height: h,
                        width: w
                    })
                }
            }
        }

        decoder.send_eof()?;
        Ok(frames)
    }

    pub fn hash(&self) -> String {
        self.local.clone()
    }

}
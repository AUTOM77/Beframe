use md5::{Md5, Digest};

use std::io::Cursor;


use av_codec::common::CodecList;
use av_codec::decoder::Decoder;
use av_data::frame::ArcFrame;
use av_data::packet::Packet;
use av_format::demuxer::{Context as DemuxerCtx, Demuxer};
use std::io::Cursor;

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
        let cursor = Cursor::new(&self.buffer);
        let mut demuxer = DemuxerCtx::new(Box::new(cursor));
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
use polars::prelude::*;
use std::io::{Read, Write};
use md5::{Md5, Digest};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Parquet {
    path: PathBuf,
}

impl Parquet {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn save_path(&self) -> PathBuf {
        self.path.with_extension("")
    }

    pub fn decode_video(&self) -> Result<Vec<Vec<u8>>, PolarsError> {
        let x: Vec<Vec<u8>> = LazyFrame::scan_parquet(&self.path, Default::default())?
            .select([col("video")])
            .collect()?
            .column("video")?
            .binary()?
            .iter()
            .filter_map(|video| video)
            .map(|x| x.to_vec())
            .collect();
        Ok(x)
    }

    pub fn save_video(&self, buffer: Vec<Vec<u8>>) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let root = self.save_path();
        std::fs::create_dir_all(&root)?;

        for buf in buffer {
            let mut hasher = Md5::new();
            hasher.update(&buf);
            let digest = hasher.finalize();
            let _name = format!("{:032x}.mp4", digest);
            let video_path = root.join(_name);

            if std::fs::metadata(&video_path).is_err() {
                let f = std::fs::File::create(&video_path)?;
                let mut w = std::io::BufWriter::new(f);
                w.write_all(&buf)?;
                w.flush()?;
            }
        }
        Ok(root)
    }

    pub fn sample(&self) -> PathBuf {
        let buffer = self.decode_video().unwrap();
        let root = self.save_video(buffer).unwrap();
        root
    }
}
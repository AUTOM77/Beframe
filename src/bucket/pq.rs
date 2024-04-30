use polars::prelude::*;
use std::path::PathBuf;
use std::io::{Read, Write};
use md5::{Md5, Digest};

pub struct Bucket {
    path: PathBuf,
    local: PathBuf
}

impl Bucket {
    pub fn from(path: PathBuf, root: PathBuf) -> Self {
        let _pth = path.clone();
        let fname = _pth.file_stem().and_then(|stem| stem.to_str()).unwrap();
        Self {
            path,
            local: root.join(fname),
        }
    }

    pub fn load(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut buff = Vec::new();

        let f = std::fs::File::open(&self.path)?;
        let mut r = std::io::BufReader::new(f);
        
        r.read_to_end(&mut buff)?;
        Ok(buff)
    }

    pub fn av_split(&self, buffer: Vec<u8>, idx: u32 ) -> Result<(), std::io::Error> {
        let path = self.local.join(format!("{:04}.mp4", idx));

        let f = std::fs::File::create(path)?;
        let mut w = std::io::BufWriter::new(f);
        
        let _ = w.write_all(&buffer)?;

        Ok(())
    }

    pub fn hash(&self) -> String {
        let buff = self.load().unwrap();
        let mut hasher = Md5::new();
        hasher.update(buff);
        let digest = hasher.finalize();
        format!("{:x}",digest)
    }

    pub fn sample(&self) -> Result<Vec<Vec<u8>>, PolarsError> {
        let _ = self.mkdir()?;
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

    pub fn sample_dry(&self) -> Result<(), PolarsError> {
        let _ = self.mkdir()?;

        let _ = LazyFrame::scan_parquet(&self.path, Default::default())?
            .select([col("video")])
            .collect()?
            .column("video")?
            .binary()?
            .iter()
            .filter_map(|video| video)
            .map(|x| x.to_vec())
            .enumerate()
            .try_for_each(|(i, video)| {
                self.av_split(video, i as u32)
            });
        Ok(())
    }

    pub fn mkdir(&self) -> Result<(), std::io::Error> {
        std::fs::create_dir_all(&self.local)?;
        Ok(())
    }

    pub fn drop(&self) -> Result<(), std::io::Error> {
        std::fs::remove_dir_all(&self.local)?;
        Ok(())
    }
}
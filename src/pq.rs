use polars::prelude::*;
use rayon::prelude::*;
use md5::{Md5, Digest};
use std::path::PathBuf;

use super::video::X264Video;
// bucket_hash/[ cache/{0..n}.mp4, frame/{0..n}.jpg ]

pub struct Bucket {
    root: PathBuf,
    path: PathBuf,
    local: PathBuf
}

impl Bucket {
    pub fn from(path: std::path::PathBuf, root: std::path::PathBuf) -> Self {
        let buff = std::fs::read(&path).unwrap();
        let mut hasher = Md5::new();
        hasher.update(buff);
        let digest = hasher.finalize();
        let local = root.join(format!("{:x}", digest));

        Self {
            root,
            path,
            local
        }
    }

    pub fn sample(&self) -> Result<Vec<X264Video>, PolarsError> {
        let _ = self.mkdir()?;
        let df: DataFrame = LazyFrame::scan_parquet(&self.path, Default::default())?
            .select([col("video")])
            .collect()?;

        let video_series = df.column("video")?.binary()?;

        let x: Vec<X264Video> = video_series
            .into_iter()
            .filter_map(|video| video)
            .par_bridge()
            .map(|x| X264Video::load(x.to_vec(), &self.local))
            .collect();
        Ok(x)
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

pub fn process_buckets_from(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let root= PathBuf::from("/dev/shm");

    let _ = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|pq| Bucket::from(pq, root.clone()))
        .for_each(|x| x.sample().expect("mkdir failed"));
    Ok(())
}
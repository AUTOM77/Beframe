// use std::io::Write;
// use polars::prelude::*;
use rayon::prelude::*;

use md5::{Md5, Digest};
use std::path::PathBuf;

pub struct Bucket {
    root: PathBuf,
    path: PathBuf,
    local: PathBuf
// bucket_hash/[ cache/{0..n}.mp4, frame/{0..n}.jpg ]
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
        .for_each(|x| x.mkdir().expect("mkdir failed"));
    Ok(())
}

// pub fn sample(pq_path: &str, batch_size: usize) -> Result<(), PolarsError> {
//     let df: DataFrame = LazyFrame::scan_parquet(pq_path, Default::default())?
//         .select([col("video")])
//         .collect()?;

//     let video_series = df.column("video")?.binary()?;

//     video_series.iter().enumerate().into_iter()
//         .par_bridge().for_each(|(i, video)| {
//             if let Some(video_data) = video {
//                 let name = format!("{:04}.mp4", i);
//                 let mut output_file = std::fs::File::create(name).unwrap();
//                 let _ = output_file.write_all(video_data);
//             }
//         });
//     Ok(())
// }

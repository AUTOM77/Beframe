use std::time::Instant;
use std::path::PathBuf;
use rayon::prelude::*;

use crate::bucket::pq::Bucket;
use crate::video::av::X264Video;

pub fn process_single_bucket(f: PathBuf) -> Result<Bucket, Box<dyn std::error::Error>> {
    let root = PathBuf::from("/dev/shm/video");
    let x = Bucket::from(f, &root);
    Ok(x)
}

pub fn process_video_join(cache: Vec<Vec<u8>>, root:PathBuf) -> Vec<X264Video> {
    cache
        .par_iter()
        .map(|c| {
            let v = X264Video::load(c.to_vec(), &root).expect("Error video");
            v.clip().expect("Error clip");
            v.drop().expect("Error drop");
            v
        })
        .collect()
}

pub fn process_video(cache: Vec<Vec<u8>>, _root:PathBuf) -> Vec<X264Video> {
    let root = PathBuf::from("/data/frame");

    cache
        .par_iter()
        .map(|c| {
            let v = X264Video::load(c.to_vec(), &root).expect("Error video");
            v.clip().expect("Error clip");
            v.drop().expect("Error drop");
            v
        })
        .collect()
}

pub fn process_video_chunks(chunks: &[Bucket]) -> Vec<Vec<X264Video>> {
    chunks
        .par_iter()
        .map(|bucket| bucket.sample_dir().expect("Error bucket"))
        .map(|(c, p) | process_video(c, p))
        .collect()
}

pub fn process_buckets_video(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("/data/frame");


    let buckets: Vec<Bucket> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|pq| Bucket::from(pq, &root) )
        .collect();
    
    let videos: Vec<Vec<Vec<X264Video>>> = buckets
            .chunks(5)
            .map(|chunk| process_video_chunks(chunk))
            .collect();
    Ok(())
}

pub fn process_buckets_bytes(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("/dev/shm/video");

    let buckets: Vec<Bucket> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|pq| Bucket::from(pq, &root) )
        .collect();

    let cache: Vec<Vec<Vec<Vec<u8>>>> = buckets
            .chunks(5)
            .map(|chunk| {
                let _cache: Vec<Vec<Vec<u8>>> = chunk.par_iter()
                .map(|bucket| bucket.sample().expect("shit"))
                .collect();
                _cache
            })
            .collect();
    Ok(())
}

pub fn process_buckets_chunk(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("/dev/shm/video");

    let buckets: Vec<Bucket> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|pq| Bucket::from(pq, &root) )
        .collect();

    let cache: Vec<Vec<PathBuf>> = buckets
            .chunks(5)
            .map(|chunk| {
                let _cache: Vec<PathBuf> = chunk.par_iter()
                .map(|bucket| bucket.sample_dry().expect("Error Sample"))
                .collect();
                _cache
            })
            .collect();
    Ok(())
}

pub fn process_buckets_mkdir(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("/dev/shm/video");

    let _ = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|pq| Bucket::from(pq, &root))
        .for_each(|x| x.mkdir().expect("mkdir failed"));
    Ok(())
}


use std::path::PathBuf;
use rayon::prelude::*;

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

fn process_videos_from(d: PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let hashes:Vec<String> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "mp4")
        .map(|path| X264Video::from(&path))
        .map(|video|video.hash())
        .collect();
    Ok(hashes)
}

fn process_videos(d: PathBuf) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let hashes:Vec<String> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "mp4")
        .map(|path| std::fs::read(&path))
        .filter_map(Result::ok)
        .map(|buffer| X264Video::load(&buffer))
        .map(|video| video.hash())
        .collect();
    Ok(hashes)
}

fn process_frames(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let frames:Vec<Beframe> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "mp4")
        .map(|path| Beframe::from(path))
        .collect();
    frames.par_iter()
        .for_each(|frame| frame.clip().expect("failed"));
    Ok(())
}

fn process_frames_dry(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let hashes:Vec<PathBuf>= std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "mp4")
        .map(|path| Beframe::from(path))
        .map(|video| video.hash())
        .collect();
    println!("{:?}", hashes);
    Ok(())
}

fn process_frames_x(d: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let framesX= std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .par_bridge()
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "mp4")
        .map(|path| Beframe::from(path))
        .for_each(|frame| frame.clip().expect("failed"));

    Ok(())
}
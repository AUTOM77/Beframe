use std::time::Instant;
use std::path::PathBuf;

use rayon::prelude::*;

mod hyper;

use hyper::X264Video;

fn single_cap(f: PathBuf){
    let start_time = Instant::now();
    let elapsed_time = start_time.elapsed();

    println!("Processing file: {:?}", f);
    println!("Processing time: {:?}", elapsed_time);
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
        .map(|video|video.hash())
        .collect();
    Ok(hashes)
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

fn hyper_cap(d: PathBuf) -> Result<(), Box<dyn std::error::Error>>  {
    let start_time = Instant::now();
    println!("Processing dir: {:?}", d);
    
    // let _hash = process_videos(d)?;
    let _hash = process_videos_from(d)?;

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
    Ok(())
}

pub fn processing(path: &str){
    let _pth = PathBuf::from(path);

    if _pth.is_file() {
        let _ = single_cap(_pth);
    } else {
        let _ = hyper_cap(_pth);
    }
}

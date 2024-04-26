use rayon::prelude::*;

use std::time::Instant;
use std::path::Path;

mod hyper;

pub fn single_cap(f: &str) {
    let start_time = Instant::now();
    let path = Path::new(f);
    let v = hyper::X264Video::load(path.to_path_buf());
    let _ = v.processing();
    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

pub fn rayon_cap(d: &str) {
    let start_time = Instant::now();

    std::fs::read_dir(d)
        .unwrap()
        .par_bridge() // Introduce parallelism
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "mp4" {
                Some(hyper::X264Video::load(path))
            } else {
                None
            }
        })
        .for_each(|v| { let _ = v.processing(); });

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}


pub fn batch_cap(d: &str) {
    let start_time = Instant::now();

    let path = Path::new(d);
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_file() && path.extension().unwrap_or_default() == "mp4" {
            let v = hyper::X264Video::load(path);
            let _ = v.processing();
        }
    }

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}


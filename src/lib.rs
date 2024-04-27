use rayon::prelude::*;

use std::time::Instant;
use std::path::PathBuf;
mod hyper;

fn list_files(folder_path: &str) -> Vec<PathBuf> {
    let mut mp4_files = Vec::new();
    for entry in std::fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "mp4" {
            mp4_files.push(path);
        }
    }
    mp4_files
}

pub fn single_cap(f: &str) {
    let start_time = Instant::now();

    let path = PathBuf::from(f);
    let v = hyper::X264Video::load(path);
    let _ = v.processing();

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

pub fn rayon_cap(d: &str) {
    let start_time = Instant::now();

    let files = list_files(d);
    let _ = files.par_iter().for_each(|f| {
        let v = hyper::X264Video::load(f.to_path_buf());
        let _ = v.processing();
    });

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}
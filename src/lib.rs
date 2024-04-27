use rayon::prelude::*;

use std::time::Instant;
use std::path::Path;
use std::path::PathBuf;

mod hyper;

pub fn single_cap(f: &str) {
    let start_time = Instant::now();
    let path = Path::new(f);
    let v = hyper::X264Video::load(path.to_path_buf());
    let _ = v.processing();
    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

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


pub fn cmp_rayon_1(d: &str) {
    let files = list_files(d);

    let _ = files.par_iter().for_each(|f| {
        let v = hyper::X264Video::load(f.to_path_buf());
        let _ = v.processing();
    });

}

pub fn cmp_rayon_2(d: &str) {
    std::fs::read_dir(d)
        .unwrap()
        .par_bridge()
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
}



fn collect_mp4_files_cc(folder_path: &str) -> Vec<PathBuf> {
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

fn collect_mp4_files_rayon(folder_path: &str) -> Vec<PathBuf> {

    std::fs::read_dir(folder_path)
        .unwrap()
        .par_bridge()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "mp4") {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

pub fn bench(d: &str) {
    let start_time = Instant::now();
    let _ = cmp_rayon_1(d);
    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?} rayon", elapsed_time);

    // let start_time = Instant::now();
    // let _ = cmp_rayon_2(d);
    // let elapsed_time = start_time.elapsed();
    // println!("Processing time: {:?} normal", elapsed_time);

}

pub fn rayon_cap(d: &str) {
    let start_time = Instant::now();

    std::fs::read_dir(d)
        .unwrap()
        .par_bridge()
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


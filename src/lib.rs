use std::time::Instant;
use std::path::PathBuf;

use rayon::prelude::*;
mod video;
mod pq;

fn single_cap(f: PathBuf){
    let root = PathBuf::from("/dd");

    let start_time = Instant::now();
    let x = pq::Bucket::from(f, root);
    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

fn hyper_cap(d: PathBuf) -> Result<(), Box<dyn std::error::Error>>  {
    let start_time = Instant::now();
    println!("Processing dir: {:?}", d);
    let root = PathBuf::from("/dd");

    let _ = pq::process_buckets_from(d, root);

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

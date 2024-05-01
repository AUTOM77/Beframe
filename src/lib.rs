pub mod bucket;
pub mod video;

pub mod op;

use std::time::Instant;
use std::path::PathBuf;

fn single_cap(f: PathBuf){
    let start_time = Instant::now();
    println!("Processing file: {:?}", f);

    let x = op::process_single_bucket(f);
    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}


fn hyper_cap(d: PathBuf) -> Result<(), Box<dyn std::error::Error>>  {
    let start_time = Instant::now();
    println!("Processing dir: {:?}", d);

    let x = op::process_buckets_video(d);

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
    Ok(())
}

pub fn processing(path: &str){
    let _pth = std::path::PathBuf::from(path);

    if _pth.is_file() {
        let _ = single_cap(_pth);
    } else {
        let _ = hyper_cap(_pth);
    }
}

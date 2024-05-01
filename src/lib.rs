use std::time::Instant;
use std::path::PathBuf;
use rayon::prelude::*;

pub mod bucket;

use bucket::pq::Bucket;

fn single_cap(f: PathBuf){
    let start_time = Instant::now();
    println!("Processing file: {:?}", f);

    let root = PathBuf::from("/data");
    let x = Bucket::from(f, root);

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

fn hyper_cap(d: PathBuf) -> Result<(), Box<dyn std::error::Error>>  {
    let start_time = Instant::now();
    println!("Processing dir: {:?}", d);
    let root = PathBuf::from("/data");

    let buckets: Vec<Bucket> = std::fs::read_dir(d)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|f| Bucket::from(f, root.clone()) )
        .collect();

    let _ = buckets
        .par_chunks(50)
        .for_each(
            |chunk|chunk.iter()
            .for_each(|x|x.sample_dry().expect("Error processing bucket"))
        );

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

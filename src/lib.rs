pub mod storage;
use image::buffer;
use rayon::prelude::*;

pub fn runtime(lfs: Vec<storage::Parquet>){
    let video_root: Vec<_> = lfs.par_iter()
        .map(|x| x.sample())
        .collect();

    println!("{:?}", video_root);
}

pub fn interface(mut pth: std::path::PathBuf, limit: Option<usize>) -> Result<(), Box<dyn std::error::Error>>{
    if pth.is_file() {
        let abs = std::fs::canonicalize(pth)?;
        pth = std::path::PathBuf::from(abs).parent().unwrap().to_path_buf();
    }

    let lfs: Vec<storage::Parquet> = std::fs::read_dir(pth)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().unwrap_or_default() == "parquet")
        .map(|f| storage::Parquet::new(f.into()))
        .collect();


    let _limit = limit.unwrap_or(5);
    let _ runtime(lfs);

    // for _lfs in lfs.chunks(limit_num) {
    //     runtime(_lfs);
    // }

    Ok(())
}
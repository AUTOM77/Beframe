use std::time::Instant;
use std::path::Path;
use std::path::PathBuf;
use tokio::task;

mod tk;

// pub fn single_cap(f: &str) {
//     let start_time = Instant::now();
//     let path = Path::new(f);
//     let v = tk::X264Video::load(path.to_path_buf());
//     let _ = v.processing();
//     let elapsed_time = start_time.elapsed();
//     println!("Processing time: {:?}", elapsed_time);
// }

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


async fn cmp_async_1(d: &str) -> Result<(), Box<dyn std::error::Error>> {
    let files = list_files(d);
    let mut pool = Vec::new();

    for f in files {
        pool.push(task::spawn(async {
            let v = tk::X264Video::load(f);
            let _  = v.mkdir().await;
        }));
    }

    for task in pool {
        task.await?;
    }

    Ok(())
}



pub async fn bench(d: &str) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    let _ = cmp_async_1(d).await?;
    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?} normal", elapsed_time);

    Ok(())
}


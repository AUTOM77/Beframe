
fn list_files(d: PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mp4_files: Vec<PathBuf> = std::fs::read_dir(d)?
        .into_iter()
        .map(|x| x.expect("mp4").path())
        .filter(|x| x.extension().unwrap_or_default() == "mp4")
        .collect();
    Ok(mp4_files)
}

fn load_buff(vec: Vec<PathBuf>) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let buffer: Vec<Vec<u8>> = vec.iter()
        .map(|x| std::fs::read(&x))
        .filter_map(Result::ok)
        .collect();
    Ok(buffer)
}

fn load_x64(buff: Vec<Vec<u8>>) -> Result<Vec<X264Video>, Box<dyn std::error::Error>> {
    let x64: Vec<X264Video> = buff.iter()
        .map(|x| X264Video::load(x))
        .collect();
    Ok(x64)
}

fn mkdir(x64: Vec<X264Video>) -> Result<(), Box<dyn std::error::Error>> {
    let _ = x64.iter()
        .for_each(|x|x.mkdir().expect("dir existed"));
    Ok(())
}

fn hash(x64: Vec<X264Video>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let _hash: Vec<String> = x64.iter()
        .map(|x| x.hash())
        .collect();
    Ok(_hash)
}

fn load_x64_rayon(buff: Vec<Vec<u8>>) -> Result<Vec<X264Video>, Box<dyn std::error::Error>> {
    let x64: Vec<X264Video> = buff.into_iter()
        .par_bridge()
        .map(|x| X264Video::load(&x))
        .collect();
    Ok(x64)
}

fn batch_cap(d: PathBuf) -> Result<(), Box<dyn std::error::Error>>  {
    let start_time = Instant::now();
    println!("Processing dir: {:?}", d);
    
    let video = list_files(d)?;
    let buff = load_buff(video)?;

    let x64 = load_x64(buff)?;
    // let x64 = load_x64_rayon(buff)?;
    // println!("Processing buff: {:?}", buff);
    // let _ = mkdir(x64)?;
    // let _hash = hash(x64)?;

    // println!("Processing hash: {:?}", _hash);

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
    Ok(())
}

use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();
    let _ = lib::runtime(cli.path.into());
    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}
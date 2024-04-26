use clap::Parser;
use std::time::Instant;

mod hyper;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'f', long, name = "FILE")]
    file: String
}

fn main() {
    let _args = Args::parse();
    let v = hyper::X264Video::load(&_args.file);
    let start_time = Instant::now(); // Start time measurement

    let _ = v.processing();

    let elapsed_time = start_time.elapsed();
    println!("Processing time: {:?}", elapsed_time);
}

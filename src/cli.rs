use clap::{Args, Parser};

#[derive(Args)]
#[group(required = false, multiple = true)]
struct Opts {
    #[arg(long, name = "LIMIT", help = "NUM of FILE limit")]
    limit: Option<usize>,
}

#[derive(Parser)]
struct Cli {
    path: String,

    #[command(flatten)]
    opt: Opts,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let cli = Cli::parse();
    let _ = lib::interface(cli.path.into(), cli.opt.limit);
    println!("Processing time: {:?}", start_time.elapsed());
    Ok(())
}
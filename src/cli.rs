use clap::{Args, Parser};

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Opts {
    #[arg(short = 'f', long, name = "FILE")]
    file: Option<String>,

    #[arg(short = 'd', long, name = "DIR")]
    dir: Option<String>,
}

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    opt: Opts,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let opt = &cli.opt;
    if let Some(file_path) = opt.file.as_deref() {
        let _ = core::pq_cap(file_path);
    } else if let Some(dir_path) = opt.dir.as_deref() {
        let _ = core::rayon_cap(dir_path);
    }

    Ok(())
}

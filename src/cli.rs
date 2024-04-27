use clap::{Args, Parser};
use tokio;

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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let opt = &cli.opt;
    if let Some(file_path) = opt.file.as_deref() {
        // let _ = core::single_cap(file_path);
    } else if let Some(dir_path) = opt.dir.as_deref() {
        let _ = core::bench(dir_path);
        // let _ = core::rayon_cap(dir_path);
        // let _ = core::batch_cap(dir_path);
    }
}

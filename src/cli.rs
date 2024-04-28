use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    core::processing(&cli.path);
    Ok(())
}

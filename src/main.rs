use clap::Parser;
use anyhow::{ Context, Result };

mod binary;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    input: std::path::PathBuf,

    #[arg(short, long, default_value = "./output.mtb")]
    output: std::path::PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("input: {:?}, output: {:?}", args.input, args.output);

    Ok(())
}

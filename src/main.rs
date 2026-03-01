use clap::{ Parser, Subcommand };
use std::path::PathBuf;
use anyhow::Result;

mod compile;
mod pack;
mod osu;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        input: PathBuf,

        #[arg(short, long, default_value = "./output.mtb")]
        output: PathBuf
    },

    Osu {
        input: PathBuf,

        #[arg(short, long, default_value = "./output.mtn")]
        output: PathBuf
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    use Commands::*;
    match cli.command {
        Compile { input, output } => {
            compile::compile(input, output)
        },
        Osu { input, output } => {
            osu::osu(input, output)
        }
    }?;

    Ok(())
}

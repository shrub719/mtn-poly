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
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compile {
        input: PathBuf,

        #[arg(short, long, default_value = "./output.mtb")]
        output: PathBuf,

        #[arg(short, long, default_value = "0")]
        start_offset: u32
    },

    Osu {
        input: PathBuf,

        #[arg(short, long, default_value = "./output.mtn")]
        output: PathBuf,

        #[arg(short, long, default_value = "0")]
        start_offset: u32
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    use Command::*;
    match cli.command {
        Compile { input, output, start_offset } => {
            compile::compile(input, output, start_offset)
        },
        Osu { input, output, start_offset } => {
            osu::osu(input, output, start_offset)
        }
    }
}

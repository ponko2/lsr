use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(
        value_name = "PATH",
        help = "Files and/or directories",
        default_value = "."
    )]
    paths: Vec<String>,

    #[arg(short, long, help = "Long listing")]
    long: bool,

    #[arg(short = 'a', long = "all", help = "Show all files")]
    show_hidden: bool,
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    dbg!(args);
    Ok(())
}

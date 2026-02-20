use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about  = None)]
pub struct Args {
    pub file: String,
}

pub fn parse() -> Args {
    Args::parse()
}

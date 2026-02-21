use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about  = None)]
pub struct Args {
    pub file: String,

    /// Render ANSI color codes
    #[arg(short, long)]
    pub color: bool,

    /// Attempt edge detection (Experimental)
    #[arg(short, long)]
    pub edge: bool,
}

pub fn parse() -> Args {
    Args::parse()
}

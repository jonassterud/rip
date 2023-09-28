use std::path::PathBuf;

pub use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path to one or more torrent files
    #[arg(short, long, value_name = "FILE(s)")]
    pub torrents: Option<Vec<PathBuf>>,
    /// Path to out directory
    #[arg(short, long, value_name = "DIR")]
    pub out: PathBuf,
}

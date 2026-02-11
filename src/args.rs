use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(
    version = "0.3", 
    about = "Archivator program based on LZ77 compressing algorithm", 
    long_about = "Copyright @ 2026 enderavour"
)]
pub struct LZRSArgs
{
    #[arg(long, short, action = ArgAction::SetTrue, help = "Compress the given files into .lzrs archive")]
    pub compress: bool,

    #[arg(long, short, action = ArgAction::SetTrue, help = "Decompress files from given .lzrs archive")]
    pub decompress: bool,

    #[arg(long, short, help = "Specify output name of archive")]
    pub output: Option<String>,

    #[arg(required = true, help = "Specify sequence of files to archive")]
    pub files: Vec<String>
}


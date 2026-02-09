mod lz77;
mod dearchive;
mod archive;
use std::error::Error;
use std::fs::File;
use archive::LZRSArchiveBuilder;
use std::env;
use memmap2::MmapOptions;

const SEARCH_BUFFER_SIZE: usize = 256;
const LOOKAHEAD_BUFFER_SIZE: i32 = 64;
const TOKEN_SIZE: u64 = 5;
fn main() -> Result<(), Box<dyn Error>>
{
    let command_line_args = env::args().collect::<Vec<String>>();
    let entered_file_name = &command_line_args[1];
    if entered_file_name.ends_with(".lzrs")
    {
        let mapped_archive = unsafe {
            MmapOptions::new().map(&File::open(entered_file_name.clone()).unwrap()).unwrap()
        };

        dearchive::extract_archive(&mapped_archive.iter().as_slice())?;
    }
    else 
    {
        let mut copied_fname = entered_file_name.clone().split(".").nth(0).unwrap().to_owned();
        copied_fname.push_str(".lzrs");

        // Some phrase with often letter repetitions
        let mut builder = LZRSArchiveBuilder::new();
        for i in 1..command_line_args.len()
        {
            builder.add_existing_file(command_line_args[i].clone());
        }

        builder.write_to_file(copied_fname.to_string())?;
    }
    
    Ok(())
}

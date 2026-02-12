use std::io::{self, Write};
use crate::{LOOKAHEAD_BUFFER_SIZE, SEARCH_BUFFER_SIZE, dearchive::IntoBytes, lz77, lz78};
use std::fs::File;
use memmap2::MmapOptions;
use crate::args::CompressingMode;

pub struct LZRSEntry
{
    pub compressed_size: u64,
    pub original_size: u64,
    pub data_offset: u64,
    pub file_name: String
}

pub struct LZRSArchiveBuilder
{
    entries: Vec<LZRSEntry>,
    compression_method: u32,
    compressed_blobs: Vec<Vec<u8>>
}

impl LZRSArchiveBuilder
{
    pub fn new() -> Self
    {
        LZRSArchiveBuilder { 
            entries: Vec::new(), 
            compression_method: 0,
            compressed_blobs: Vec::new() 
        }
    }

    pub fn set_compression_method(&mut self, compressing_mode: CompressingMode)
    {
        self.compression_method = match compressing_mode 
        {
            CompressingMode::LZ77 => 77,
            CompressingMode::LZ78 => 78
        };
    }

    fn header_size(&self) -> u64
    {
        let mut size = 0;

        size += 4; // signature
        size += 4; // compression method (LZ77 or LZ78)
        size += 8; // entries count
        for entry in self.entries.iter()
        {
            size += 8 + 8 + 8 + 8 + entry.file_name.len() as u64;
        }
        size
    }

    fn finalize_offsets(&mut self)
    {
        let mut offset = self.header_size();

        for entry in self.entries.iter_mut()
        {
            entry.data_offset = offset;
            offset += entry.compressed_size;
        }
    }

    pub fn add_file(&mut self, name: String, data: &[u8], compressing_mode: CompressingMode)
    {
        let compressed_data: Vec<u8>; 
        
        match compressing_mode 
        {
            CompressingMode::LZ77 => 
            {
                compressed_data = lz77::compress(
             data, 
            SEARCH_BUFFER_SIZE, 
        LOOKAHEAD_BUFFER_SIZE
                ).to_bytes();
            }

            CompressingMode::LZ78 => 
            {
                compressed_data = lz78::compress(data).to_bytes();
            }
        }

        self.entries.push(LZRSEntry { 
            compressed_size: compressed_data.len() as u64,
            original_size: data.len() as u64,
            data_offset: 0,
            file_name: name
        });

        self.compressed_blobs.push(compressed_data);
    }

    pub fn add_existing_file(&mut self, filename: String, compressing_mode: CompressingMode)
    {
        let mapped_file = unsafe { 
            MmapOptions::new().map(&File::open(filename.clone()).unwrap()).unwrap()
        };
        self.add_file(filename, mapped_file.iter().as_slice(), compressing_mode);
    }

    pub fn write<W: Write>(&mut self, mut w: W) -> io::Result<()>
    {
        self.finalize_offsets();

        // Serialization of header

        w.write_all(b"LZRS")?; // Signature
        w.write_all(&self.compression_method.to_le_bytes())?; // Compression Method
        w.write_all(&(self.entries.len() as u64).to_le_bytes())?; // Entries count

        for e in self.entries.iter()
        {
            w.write_all(&e.compressed_size.to_le_bytes())?;
            w.write_all(&e.original_size.to_le_bytes())?;
            w.write_all(&e.data_offset.to_le_bytes())?;

            let file_name = e.file_name.as_bytes();
            let name_len = file_name.len() as u64;
            w.write_all(&name_len.to_le_bytes())?;
            w.write_all(file_name)?;
        }

        for blob in self.compressed_blobs.iter()
        {
            w.write_all(blob)?;
        }

        Ok(())
    }

    pub fn write_to_file(&mut self, filename: String) -> io::Result<()>
    {
        let fh = File::create(filename)?;
        self.write(fh)?;
        Ok(())
    }
}
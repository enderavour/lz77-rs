# lzrs - Implementation of LZ77-based archivator made in Rust

### Description
- Project is not yet ready, needs to be fixed with dearchived context of extracted files (They repair in their original structure, but several trash bytes are added -> PDF files appear to be corrupted in such case)
- Command line argument parsing will be added later, currently archivation and dearchivation process are implemented based on file extension
- To download and run the project:
```
git clone https://github.com/enderavour/lz77-rs.git
```
```
cd lz77-rs
```
```
cargo run -- file1.txt file2.txt 
```
For creating archive (.lzrs file)
```
cargo run -- file.lzrs
```
For dearchivation (extracting files from .lzrs)
- The source code of LZ77 compression and decompression algorithms was partially inspired by and rewritten from [wolfie-things/lz77-algorithm](https://github.com/wolfie-things/lz77-algorithm)
- Contributors are welcomed.

### Used crates
- [memmap2](https://crates.io/crates/memmap2) - A Rust library for cross-platform memory mapped IO.

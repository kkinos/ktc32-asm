use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufRead, BufReader, Write};

mod converter;
mod parser;
use converter::{convert, Word};
use parser::parse;

#[derive(Parser)]
#[clap(version = "0.1", author = "kinpoko", about = "ktc32 assembler")]
struct Cli {
    /// .asm file path
    file_path: std::path::PathBuf,

    /// output file name
    #[arg(short = 'o', default_value = "a.mem")]
    output_file: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.file_path)
        .with_context(|| format!("could not read file '{}'", &args.file_path.display()))?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let format = parse(line)?;
        convert(&format)?;
    }

    let file = std::fs::File::open(&args.file_path)
        .with_context(|| format!("could not read file '{}'", &args.file_path.display()))?;
    let reader = BufReader::new(file);

    let mut new_file = std::fs::File::create(&args.output_file)
        .with_context(|| format!("could not create file"))?;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            continue;
        }
        let format = parse(line)?;
        let word = convert(&format)?;
        match word {
            Word::Word16(n) => {
                writeln!(new_file, "{:02x}", n & 0x00FF)?;
                writeln!(new_file, "{:02x}", (n & 0xFF00) >> 8)?;
            }
            Word::Word32(n) => {
                writeln!(new_file, "{:02x}", n & 0x0000_00FF)?;
                writeln!(new_file, "{:02x}", (n & 0x0000_FF00) >> 8)?;
                writeln!(new_file, "{:02x}", (n & 0x00FF_0000) >> 16)?;
                writeln!(new_file, "{:02x}", (n & 0xFF00_0000) >> 24)?;
            }
        }
    }

    Ok(())
}

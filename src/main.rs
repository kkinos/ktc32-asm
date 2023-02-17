use anyhow::{Context, Result};
use clap::Parser;
use std::io::{BufReader, Write};

mod converter;
mod parser;
use converter::{convert, Word};
use parser::parse;

#[derive(Parser)]
#[clap(version = "0.1", author = "kinpoko", about = "KTC32 assembler")]
struct Cli {
    /// .asm file path
    file_path: std::path::PathBuf,

    /// output file name
    #[arg(short = 'o', default_value = "a.mem")]
    output_file: std::path::PathBuf,

    /// assemble for bare metal
    #[arg(short = 'b', long = "baremetal")]
    bare_metal: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let file = std::fs::File::open(&args.file_path)
        .with_context(|| format!("could not read file '{}'", &args.file_path.display()))?;
    let reader = BufReader::new(file);

    let (parsed_lines, symbol_table, ..) = parse(reader, args.bare_metal)?;
    for parsed_line in parsed_lines {
        convert(&parsed_line, &symbol_table)?;
    }

    let file = std::fs::File::open(&args.file_path)
        .with_context(|| format!("could not read file '{}'", &args.file_path.display()))?;
    let reader = BufReader::new(file);

    let mut new_file = std::fs::File::create(&args.output_file)
        .with_context(|| "could not create file".to_string())?;

    let (parsed_lines, symbol_table, bytes) = parse(reader, args.bare_metal)?;
    if !args.bare_metal {
        writeln!(new_file, "{:02x}", bytes & 0x0000_00FF)?;
        writeln!(new_file, "{:02x}", (bytes & 0x0000_FF00) >> 8)?;
        writeln!(new_file, "{:02x}", (bytes & 0x00FF_0000) >> 16)?;
        writeln!(new_file, "{:02x}", (bytes & 0xFF00_0000) >> 24)?;
    }
    for parsed_line in parsed_lines {
        let word = convert(&parsed_line, &symbol_table)?;

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

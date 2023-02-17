use anyhow::{anyhow, Ok, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum Format {
    RFormat {
        mnemonic: String,
        rd: String,
        rs: String,
        address: i64,
        line_num: u64,
    },
    I16Format {
        mnemonic: String,
        rd: String,
        imm: String,
        address: i64,
        line_num: u64,
    },
    I32Format {
        mnemonic: String,
        rd: String,
        rs: String,
        imm: String,
        address: i64,
        line_num: u64,
    },
    JFormat {
        mnemonic: String,
        rd: String,
        imm: String,
        address: i64,
        line_num: u64,
    },
    Const {
        num: String,
        address: i64,
        line_num: u64,
    },
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub address: i64,
}

pub fn parse(reader: BufReader<File>, bare_metal: bool) -> Result<(Vec<Format>, Vec<Symbol>, u32)> {
    let mut parsed_lines: Vec<Format> = Vec::new();
    let mut symbol_table: Vec<Symbol> = Vec::new();
    let mut address: i64 = if bare_metal { 0 } else { 256 };

    let mut line_num = 0;
    for line in reader.lines() {
        line_num += 1;
        let line = line.unwrap();
        if line.trim().len() == 0 || line.trim().starts_with("//") {
            continue;
        }

        let line = line.replace(',', " ");
        let line = line
            .split_ascii_whitespace()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<_>>();

        match line[0].as_str() {
            "mov" | "add" | "sub" | "and" | "or" | "xor" | "sll" | "srl" | "sra" | "slt"
            | "sltu" => {
                if line.len() == 3 {
                    parsed_lines.push(Format::RFormat {
                        mnemonic: line[0].to_string(),
                        rd: line[1].to_string(),
                        rs: line[2].to_string(),
                        address: address,
                        line_num: line_num,
                    });
                    address += 2;
                } else {
                    Err(anyhow!("line {} syntax error {:?}", line_num, line))?
                }
            }

            "slli" | "srli" | "srai" => {
                if line.len() == 3 {
                    parsed_lines.push(Format::I16Format {
                        mnemonic: line[0].to_string(),
                        rd: line[1].to_string(),
                        imm: line[2].to_string(),
                        address: address,
                        line_num: line_num,
                    });
                    address += 2;
                } else {
                    Err(anyhow!("line {} syntax error {:?}", line_num, line))?
                }
            }

            "addi" | "andi" | "ori" | "xori" | "slti" | "sltiu" | "beq" | "bnq" | "blt" | "bge"
            | "bltu" | "bgeu" | "jalr" | "lb" | "lh" | "lbu" | "lhu" | "lw" | "sb" | "sh"
            | "sw" => {
                if line.len() == 4 {
                    parsed_lines.push(Format::I32Format {
                        mnemonic: line[0].to_string(),
                        rd: line[1].to_string(),
                        rs: line[2].to_string(),
                        imm: line[3].to_string(),
                        address: address,
                        line_num: line_num,
                    });
                    address += 4;
                } else {
                    Err(anyhow!("line {} syntax error {:?}", line_num, line))?
                }
            }
            "lui" => {
                if line.len() == 3 {
                    parsed_lines.push(Format::I32Format {
                        mnemonic: line[0].to_string(),
                        rd: line[1].to_string(),
                        rs: "r0".to_string(),
                        imm: line[2].to_string(),
                        address: address,
                        line_num: line_num,
                    });
                    address += 4;
                } else {
                    Err(anyhow!("line {} syntax error {:?}", line_num, line))?
                }
            }
            "jal" => {
                if line.len() == 3 {
                    parsed_lines.push(Format::JFormat {
                        mnemonic: line[0].to_string(),
                        rd: line[1].to_string(),
                        imm: line[2].to_string(),
                        address: address,
                        line_num: line_num,
                    });
                    address += 4;
                } else {
                    Err(anyhow!("line {} syntax error {:?}", line_num, line))?
                }
            }
            _ => {
                if line.len() == 1 && line[0].ends_with(":") {
                    symbol_table.push(Symbol {
                        name: line[0].trim_end_matches(":").to_string(),
                        address: address,
                    });
                } else if line.len() == 1 && line[0].starts_with("0x") {
                    parsed_lines.push(Format::Const {
                        num: line[0].to_string(),
                        address: address,
                        line_num: line_num,
                    });
                    address += 4;
                } else {
                    Err(anyhow!("line {} syntax error {:?}", line_num, line))?
                }
            }
        }
    }

    let bytes: u32 = if bare_metal {
        (address) as u32
    } else {
        (address as u32) - 256
    };

    Ok((parsed_lines, symbol_table, bytes))
}

use anyhow::{anyhow, Ok, Result};
#[derive(Debug)]
pub enum Format {
    RFormat {
        mnemonic: String,
        rd: String,
        rs: String,
    },
    I16Format {
        mnemonic: String,
        rd: String,
        imm: String,
    },
    I32Format {
        mnemonic: String,
        rd: String,
        rs: String,
        imm: String,
    },
    JFormat {
        mnemonic: String,
        rd: String,
        imm: String,
    },
}

pub fn parse(line: String) -> Result<Format, anyhow::Error> {
    let line = line.replace(',', " ");

    let line = line
        .split_whitespace()
        .map(|s| s.to_ascii_lowercase())
        .collect::<Vec<_>>();

    match line[0].as_str() {
        "mov" | "add" | "sub" | "and" | "or" | "xor" | "sll" | "srl" | "sra" | "slt" | "sltu" => {
            if line.len() == 3 {
                Ok(Format::RFormat {
                    mnemonic: line[0].to_string(),
                    rd: line[1].to_string(),
                    rs: line[2].to_string(),
                })
            } else {
                Err(anyhow!("syntax error {:?}", line))
            }
        }

        "slli" | "srli" | "srai" => {
            if line.len() == 3 {
                Ok(Format::I16Format {
                    mnemonic: line[0].to_string(),
                    rd: line[1].to_string(),
                    imm: line[2].to_string(),
                })
            } else {
                Err(anyhow!("syntax error {:?}", line))
            }
        }

        "addi" | "andi" | "ori" | "xori" | "beq" | "bnq" | "blt" | "bge" | "bltu" | "bgeu"
        | "jalr" | "lb" | "lh" | "lbu" | "lhu" | "lw" | "lui" | "sb" | "sh" | "sw" => {
            if line.len() == 4 {
                Ok(Format::I32Format {
                    mnemonic: line[0].to_string(),
                    rd: line[1].to_string(),
                    rs: line[2].to_string(),
                    imm: line[3].to_string(),
                })
            } else {
                Err(anyhow!("syntax error {:?}", line))
            }
        }
        "jal" => {
            if line.len() == 3 {
                Ok(Format::JFormat {
                    mnemonic: line[0].to_string(),
                    rd: line[1].to_string(),
                    imm: line[2].to_string(),
                })
            } else {
                Err(anyhow!("syntax error {:?}", line))
            }
        }
        _ => Err(anyhow!("unknown mnemonic {}", line[0])),
    }
}

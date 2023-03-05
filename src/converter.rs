use crate::parser::{Symbol, Type};
use anyhow::{anyhow, Context, Ok, Result};

#[derive(Debug)]
pub enum Word {
    Word16(u16),
    Word32(u32),
}

pub fn convert(parsed_line: &Type, symbol_table: &Vec<Symbol>) -> Result<Word> {
    let opcode = match parsed_line {
        Type::RFormat {
            mnemonic, line_num, ..
        } => match mnemonic.as_str() {
            "mov" => Ok(0b000000),
            "add" => Ok(0b000001),
            "sub" => Ok(0b000010),
            "and" => Ok(0b000011),
            "or" => Ok(0b000100),
            "xor" => Ok(0b000101),
            "sll" => Ok(0b000110),
            "srl" => Ok(0b000111),
            "sra" => Ok(0b001000),
            "slt" => Ok(0b001001),
            "sltu" => Ok(0b001010),
            _ => Err(anyhow!("line {} unknown mnemonic {}", line_num, mnemonic))?,
        },
        Type::I16Format {
            mnemonic, line_num, ..
        } => match mnemonic.as_str() {
            "slli" => Ok(0b010000),
            "srli" => Ok(0b010001),
            "srai" => Ok(0b010010),
            _ => Err(anyhow!("line {} unknown mnemonic {}", line_num, mnemonic))?,
        },
        Type::I32Format {
            mnemonic, line_num, ..
        } => match mnemonic.as_str() {
            "addi" => Ok(0b100000),
            "andi" => Ok(0b100001),
            "ori" => Ok(0b100010),
            "xori" => Ok(0b100011),
            "slti" => Ok(0b100100),
            "sltiu" => Ok(0b100101),
            "beq" => Ok(0b100110),
            "bnq" => Ok(0b100111),
            "blt" => Ok(0b101000),
            "bge" => Ok(0b101001),
            "bltu" => Ok(0b101010),
            "bgeu" => Ok(0b101011),
            "jalr" => Ok(0b101100),
            "lb" => Ok(0b101101),
            "lh" => Ok(0b101110),
            "lbu" => Ok(0b101111),
            "lhu" => Ok(0b110000),
            "lw" => Ok(0b110001),
            "lui" => Ok(0b110010),
            "sb" => Ok(0b110011),
            "sh" => Ok(0b110100),
            "sw" => Ok(0b110101),
            _ => Err(anyhow!("line {} unknown mnemonic {}", line_num, mnemonic))?,
        },
        Type::JFormat {
            mnemonic, line_num, ..
        } => match mnemonic.as_str() {
            "jal" => Ok(0b111111),
            _ => Err(anyhow!("line {} unknown mnemonic {}", line_num, mnemonic))?,
        },
        _ => Ok(0),
    };

    let rd = match parsed_line {
        Type::RFormat { rd, line_num, .. }
        | Type::I16Format { rd, line_num, .. }
        | Type::I32Format { rd, line_num, .. }
        | Type::JFormat { rd, line_num, .. } => match rd.as_str() {
            "r0" | "zero" => Ok(0b00000),
            "r1" | "ra" => Ok(0b00001),
            "r2" | "gp" => Ok(0b00010),
            "r3" | "sp" => Ok(0b00011),
            "r4" | "fp" => Ok(0b00100),
            "r5" | "a0" => Ok(0b00101),
            "r6" | "a1" => Ok(0b00110),
            "r7" | "a2" => Ok(0b00111),
            "r8" | "a3" => Ok(0b01000),
            "r9" | "t0" => Ok(0b01001),
            "r10" | "t1" => Ok(0b01010),
            "r11" | "t2" => Ok(0b01011),
            "r12" | "t3" => Ok(0b01100),
            "r13" | "t4" => Ok(0b01101),
            "r14" => Ok(0b01110),
            "r15" => Ok(0b01111),
            "r16" => Ok(0b10000),
            "r17" => Ok(0b10001),
            "r18" => Ok(0b10010),
            "r19" => Ok(0b10011),
            "r20" => Ok(0b10100),
            "r21" => Ok(0b10101),
            "r22" => Ok(0b10110),
            "r23" => Ok(0b10111),
            "r24" => Ok(0b11000),
            "r25" => Ok(0b11001),
            "r26" => Ok(0b11010),
            "r27" => Ok(0b11011),
            "r28" => Ok(0b11100),
            "r29" => Ok(0b11101),
            "r30" => Ok(0b11110),
            "r31" | "flag" => Ok(0b11111),
            _ => Err(anyhow!("line {} unknown register {}", line_num, rd))?,
        },
        _ => Ok(0),
    };

    let rs = match parsed_line {
        Type::RFormat { rs, line_num, .. } | Type::I32Format { rs, line_num, .. } => {
            match rs.as_str() {
                "r0" | "zero" => Ok(0b00000),
                "r1" | "ra" => Ok(0b00001),
                "r2" | "gp" => Ok(0b00010),
                "r3" | "sp" => Ok(0b00011),
                "r4" | "fp" => Ok(0b00100),
                "r5" | "a0" => Ok(0b00101),
                "r6" | "a1" => Ok(0b00110),
                "r7" | "a2" => Ok(0b00111),
                "r8" | "a3" => Ok(0b01000),
                "r9" | "t0" => Ok(0b01001),
                "r10" | "t1" => Ok(0b01010),
                "r11" | "t2" => Ok(0b01011),
                "r12" | "t3" => Ok(0b01100),
                "r13" | "t4" => Ok(0b01101),
                "r14" => Ok(0b01110),
                "r15" => Ok(0b01111),
                "r16" => Ok(0b10000),
                "r17" => Ok(0b10001),
                "r18" => Ok(0b10010),
                "r19" => Ok(0b10011),
                "r20" => Ok(0b10100),
                "r21" => Ok(0b10101),
                "r22" => Ok(0b10110),
                "r23" => Ok(0b10111),
                "r24" => Ok(0b11000),
                "r25" => Ok(0b11001),
                "r26" => Ok(0b11010),
                "r27" => Ok(0b11011),
                "r28" => Ok(0b11100),
                "r29" => Ok(0b11101),
                "r30" => Ok(0b11110),
                "r31" | "flag" => Ok(0b11111),
                _ => Err(anyhow!("line {} unknown register {}", line_num, rs))?,
            }
        }
        _ => Ok(0),
    };

    let imm = match parsed_line {
        Type::I16Format { imm, line_num, .. } => {
            let imm = if imm.starts_with("0x") {
                u8::from_str_radix(imm.trim_start_matches("0x"), 16)
                    .with_context(|| format!("line {} invalid immediate num {}", line_num, imm))?
            } else {
                imm.parse::<u8>()
                    .with_context(|| format!("line {} invalid immediate num {}", line_num, imm))?
            };
            if imm <= 0b11111 {
                Ok(imm as u32)
            } else {
                Err(anyhow!(
                    "line {} immediate num is too large {}",
                    line_num,
                    imm
                ))?
            }
        }
        Type::I32Format { imm, line_num, .. } => {
            let imm = if imm.starts_with("0x") {
                u16::from_str_radix(imm.trim_start_matches("0x"), 16)
                    .with_context(|| format!("line {} invalid immediate num {}", line_num, imm))?
            } else {
                let mut t = imm.clone();
                for symbol in symbol_table {
                    if imm == &symbol.name {
                        t = format!("{}", symbol.address);
                    }
                }
                t.parse::<i16>()
                    .with_context(|| format!("line {} invalid immediate num {}", line_num, imm))?
                    as u16
            };

            Ok(imm as u32)
        }
        Type::JFormat {
            imm,
            address,
            line_num,
            ..
        } => {
            let imm = if imm.starts_with("0x") {
                u32::from_str_radix(imm.trim_start_matches("0x"), 16)
                    .with_context(|| format!("line {} invalid immediate num {}", line_num, imm))?
            } else {
                let mut t = imm.clone();
                for symbol in symbol_table {
                    if imm == &symbol.name {
                        t = format!("{}", symbol.address - address - 4);
                    }
                }
                t.parse::<i32>()
                    .with_context(|| format!("line {} invalid immediate num {}", line_num, imm))?
                    as u32
            };
            if -(2_i32.pow(19)) < (imm as i32) && (imm as i32) < (2_i32.pow(19) - 1) {
                Ok(imm)
            } else {
                Err(anyhow!("line{} invalid immediate num {}", line_num, imm))?
            }
        }
        _ => Ok(0),
    };

    let opcode = opcode.unwrap();
    let rd = rd.unwrap();
    let rs = rs.unwrap();
    let imm = imm.unwrap();

    match parsed_line {
        Type::RFormat { .. } => {
            let line: u16 = ((opcode & 0x003F) as u16)
                | ((rd & 0x001F) as u16) << 6
                | ((rs & 0x001F) as u16) << 11;
            Ok(Word::Word16(line))
        }
        Type::I16Format { .. } => {
            let line: u16 = ((opcode & 0x003F) as u16)
                | ((rd & 0x001F) as u16) << 6
                | ((imm & 0x001F) as u16) << 11;
            Ok(Word::Word16(line))
        }
        Type::I32Format { .. } => {
            let line: u32 = ((opcode & 0x0000_003F) as u32)
                | ((rd & 0x0000_001F) as u32) << 6
                | ((rs & 0x0000_001F) as u32) << 11
                | (imm & 0x0000_FFFF) << 16;
            Ok(Word::Word32(line))
        }
        Type::JFormat { .. } => {
            let line: u32 = ((opcode & 0x0000_003F) as u32)
                | ((rd & 0x0000_001F) as u32) << 6
                | (imm & 0x001F_FFFF) << 11;
            Ok(Word::Word32(line))
        }
        Type::Const { num, line_num, .. } => {
            let line: u32 = u32::from_str_radix(num.trim_start_matches("0x"), 16)
                .with_context(|| format!("line {} invalid const num {}", line_num, num))?;
            Ok(Word::Word32(line))
        }
    }
}

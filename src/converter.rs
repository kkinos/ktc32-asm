use crate::parser::Format;
use anyhow::{anyhow, Context, Ok};

#[derive(Debug)]
pub enum Word {
    Word16(u16),
    Word32(u32),
}

pub fn convert(format: &Format) -> Result<Word, anyhow::Error> {
    let opcode = match format {
        Format::RFormat { mnemonic, .. } => match mnemonic.as_str() {
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
            _ => Err(anyhow!("unknown mnemonic {}", mnemonic)),
        },
        Format::IFormat16 { mnemonic, .. } => match mnemonic.as_str() {
            "slli" => Ok(0b010000),
            "srli" => Ok(0b010001),
            "srai" => Ok(0b010010),
            _ => Err(anyhow!("unknown mnemonic {}", mnemonic)),
        },
        Format::IFormat32 { mnemonic, .. } => match mnemonic.as_str() {
            "addi" => Ok(0b100000),
            "andi" => Ok(0b100001),
            "ori" => Ok(0b100010),
            "xori" => Ok(0b100011),
            "beq" => Ok(0b100100),
            "bnq" => Ok(0b100101),
            "blt" => Ok(0b100110),
            "bge" => Ok(0b100111),
            "bltu" => Ok(0b101000),
            "bgeu" => Ok(0b101001),
            "jalr" => Ok(0b101010),
            "lb" => Ok(0b101011),
            "lh" => Ok(0b101100),
            "lbu" => Ok(0b101101),
            "lhu" => Ok(0b101110),
            "lw" => Ok(0b101111),
            "lui" => Ok(0b110000),
            "sb" => Ok(0b110001),
            "sh" => Ok(0b110010),
            "sw" => Ok(0b110011),
            _ => Err(anyhow!("unknown mnemonic {}", mnemonic)),
        },
        Format::JFormat { mnemonic, .. } => match mnemonic.as_str() {
            "jal" => Ok(0b111111),
            _ => Err(anyhow!("unknown mnemonic {}", mnemonic)),
        },
    };

    let rd = match format {
        Format::RFormat { rd, .. }
        | Format::IFormat16 { rd, .. }
        | Format::IFormat32 { rd, .. }
        | Format::JFormat { rd, .. } => match rd.as_str() {
            "r0" => Ok(0b00000),
            "r1" => Ok(0b00001),
            "r2" => Ok(0b00010),
            "r3" => Ok(0b00011),
            "r4" => Ok(0b00100),
            "r5" => Ok(0b00101),
            "r6" => Ok(0b00110),
            "r7" => Ok(0b00111),
            "r8" => Ok(0b01000),
            "r9" => Ok(0b01001),
            "r10" => Ok(0b01010),
            "r11" => Ok(0b01011),
            "r12" => Ok(0b01100),
            "r13" => Ok(0b01101),
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
            "r31" => Ok(0b11111),
            _ => Err(anyhow!("unknown register {}", rd)),
        },
    };

    let rs = match format {
        Format::RFormat { rs, .. } | Format::IFormat32 { rs, .. } => match rs.as_str() {
            "r0" => Ok(0b00000),
            "r1" => Ok(0b00001),
            "r2" => Ok(0b00010),
            "r3" => Ok(0b00011),
            "r4" => Ok(0b00100),
            "r5" => Ok(0b00101),
            "r6" => Ok(0b00110),
            "r7" => Ok(0b00111),
            "r8" => Ok(0b01000),
            "r9" => Ok(0b01001),
            "r10" => Ok(0b01010),
            "r11" => Ok(0b01011),
            "r12" => Ok(0b01100),
            "r13" => Ok(0b01101),
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
            "r31" => Ok(0b11111),
            _ => Err(anyhow!("unknown register {}", rs)),
        },
        Format::IFormat16 { .. } | Format::JFormat { .. } => Ok(0),
    };

    let imm = match format {
        Format::IFormat16 { imm, .. } => {
            let imm = imm
                .parse::<u8>()
                .with_context(|| format!("could not parse {}", imm))?;
            if imm <= 0b11111 {
                Ok(imm as u32)
            } else {
                Err(anyhow!("invalid immidiate num {}", imm))
            }
        }
        Format::IFormat32 { imm, .. } => {
            let imm = imm
                .parse::<i16>()
                .with_context(|| format!("invalid imidiate num {}", imm))?;

            Ok(imm as u32)
        }
        Format::JFormat { imm, .. } => {
            let imm = imm
                .parse::<i32>()
                .with_context(|| format!("could not parse {}", imm))?;
            if -(2_i32.pow(19)) < imm && imm < (2_i32.pow(19) - 1) {
                Ok(imm as u32)
            } else {
                Err(anyhow!("invalid immidiate num {}", imm))
            }
        }
        Format::RFormat { .. } => Ok(0),
    };

    let opcode = opcode.unwrap();
    let rd = rd.unwrap();
    let rs = rs.unwrap();
    let imm = imm.unwrap();

    match format {
        Format::RFormat { .. } => {
            let line: u16 = ((opcode & 0x003F) as u16)
                | ((rd & 0x001F) as u16) << 6
                | ((rs & 0x001F) as u16) << 11;
            Ok(Word::Word16(line))
        }
        Format::IFormat16 { .. } => {
            let line: u16 = ((opcode & 0x003F) as u16)
                | ((rd & 0x001F) as u16) << 6
                | ((imm & 0x001F) as u16) << 11;
            Ok(Word::Word16(line))
        }
        Format::IFormat32 { .. } => {
            let line: u32 = ((opcode & 0x0000_003F) as u32)
                | ((rd & 0x0000_001F) as u32) << 6
                | ((rs & 0x0000_001F) as u32) << 11
                | ((imm & 0x0000_FFFF) as u32) << 16;
            Ok(Word::Word32(line))
        }
        Format::JFormat { .. } => {
            let line: u32 = ((opcode & 0x0000_003F) as u32)
                | ((rd & 0x0000_001F) as u32) << 6
                | ((imm & 0x001F_FFFF) as u32) << 11;
            Ok(Word::Word32(line))
        }
    }
}

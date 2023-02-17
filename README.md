# ktc32-asm

[![Rust](https://github.com/kinpoko/ktc32-asm/actions/workflows/rust.yml/badge.svg)](https://github.com/kinpoko/ktc32-asm/actions/workflows/rust.yml)
![License](https://img.shields.io/github/license/kinpoko/ktc32-asm?color=blue)

[KTC32](https://github.com/kinpoko/ktc32) assembler written in Rust.

## Build

```bash
git clone https://github.com/kinpoko/ktc32-asm.git
cargo build --release
```

## Usage

```bash
ktc32-asm -h
KTC32 assembler

Usage: ktc32-asm [OPTIONS] <FILE_PATH>

Arguments:
  <FILE_PATH>  .asm file path

Options:
  -o <OUTPUT_FILE>      output file name [default: a.mem]
  -b, --baremetal       assemble for bare metal
  -h, --help            Print help information
  -V, --version         Print version information
```

## Feature

- Comment

```asm
// loop
jal r0, -4
```

- Label

```asm
start:
	jal r0, label

label:
	addi r1, r0, 1
```

- Constant

```asm
start:
	addi r1, r0, msg
	lw r2, r1, 0

msg:
	0x6c6c6548
```

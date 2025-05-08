use std::{env, io, process};
use std::fs::File;
use std::io::Seek;

use address::*;
use arg::*;
use instr::*;
use opcode::*;
use reader::*;

pub mod reader;
pub mod arg;
pub mod address;
pub mod instr;
pub mod opcode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <RAM path> <script address>", &args[0]);
        process::exit(1);
    }

    let path = &args[1];
    let addr = u32::from_str_radix(&args[2], 16)
        .expect("Invalid address");

    let base_addr = 0x8000_0000;
    let offset= addr - base_addr;

    let mut ram = File::open(path)
        .expect("RAM dump not found");
    
    ram.seek(io::SeekFrom::Start(offset as u64))
        .expect("Failed to seek");

    dbg!(disassemble(&mut ram).expect("Couldn't read"));

    // https://github.com/encounter/decomp-toolkit/blob/18987ed330db864b48886b44a3d7fb222857e7e1/src/util/dol.rs#L147
}

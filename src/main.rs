use std::io;
use std::fs::File;
use std::io::{Read, Seek};

use csv;

use address::*;
use arg::*;
use instr::*;
use opcode::*;
use reader::*;
use printing::*;

pub mod reader;
pub mod arg;
pub mod address;
pub mod instr;
pub mod opcode;
pub mod printing;

fn round_trip<R>(reader: &mut R, addr: u32) -> bool
where
    R: Read + Seek + ?Sized
{
    let base_addr = 0x8000_0000;
    let offset= addr - base_addr;

    reader.seek(io::SeekFrom::Start(offset as u64)).expect("Failed to seek");
    let dis = disassemble(reader).expect("Couldn't read");

    let mut x = io::Cursor::new(Vec::new());
    assemble(&mut x, &dis).expect("Failed to assemble");
    x.seek(io::SeekFrom::Start(0)).expect("Failed to re-seek");
    let dis2 = disassemble(&mut x).expect("Couldn't re-read");

    return dis == dis2;
}

fn test_all<R>(reader: &mut R) -> io::Result<()>
where
    R: Read + Seek + ?Sized
{
    let mut csv = csv::Reader::from_path("../spm-docs/misc/dolscriptlocs.csv").expect("");
    for result in csv.records() {
        let record = result?;
        let addr = u32::from_str_radix(&record[0], 16).expect("Parse failed");
        assert!(round_trip(reader, addr), "Failed 0x{:x}", addr);
    }

    Ok(())
}

fn test_single<R>(reader: &mut R, addr: u32) -> io::Result<Script>
where
    R: Read + Seek + ?Sized
{
    let base_addr = 0x8000_0000;
    let offset= addr - base_addr;

    reader.seek(io::SeekFrom::Start(offset as u64)).expect("Failed to seek");
    let dis = disassemble(reader).expect("Couldn't read");

    let mut x = io::Cursor::new(Vec::new());
    assemble(&mut x, &dis).expect("Failed to assemble");
    x.seek(io::SeekFrom::Start(0)).expect("Failed to re-seek");
    return disassemble(&mut x);
}

fn main() {
    // let args: Vec<String> = env::args().collect();

    // if args.len() < 3 {
    //     eprintln!("Usage: {} <RAM path> <script address>", &args[0]);
    //     process::exit(1);
    // }

    // let path = &args[1];
    let path = "../evt-disassembler/ram.raw";
    let mut ram = File::open(path).expect("RAM dump not found");

    // test_all(&mut ram).expect("Test failed");
    let mut out = String::new();
    print_evt(
        &mut out,
        test_single(&mut ram, 0x803fbd9c).expect(""),
        // PrintSettings::default()
        PrintSettings{macros: true}
    ).expect("");
    println!("{}", out);


    // https://github.com/encounter/decomp-toolkit/blob/18987ed330db864b48886b44a3d7fb222857e7e1/src/util/dol.rs#L147
}

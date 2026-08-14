use std::env;
use std::fs::File;
use std::io;
use std::io::Seek;
use std::net::IpAddr;
use std::process;

use address::*;
use arg::*;
use instr::*;
use opcode::*;
use printing::*;

pub mod address;
pub mod arg;
pub mod instr;
pub mod opcode;
pub mod printing;
pub mod ipcclient;
pub mod reader;

// TODO: anyhow

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let usage = || {
        eprintln!("
Usage: {0} disassemble <path> <address>
       {0} network <ip> <port>
       {0} help
            ",
             &args[0]
        );
    };

    let usage_fail = || {
        usage();
        Err("Invalid arguments".into())
    };

    if args.len() < 2 {
        return usage_fail()
    }

    let cmd = &args[1];
    let args = &args[2..];

    match cmd.as_str() {
        "disassemble" => {
            if args.len() < 2 {
                return usage_fail()
            }

            let path = &args[0];
            let mut ram = File::open(path)?;

            let addr = args[1].trim_start_matches("0x");
            let addr = u32::from_str_radix(addr, 16)?;

            let base_addr: u32 = 0x8000_0000;
            if base_addr > addr {
                eprintln!("Invalid address");
                process::exit(1)
            }

            let offset = addr - base_addr;

            ram.seek(io::SeekFrom::Start(offset as u64))?;

            let dis = disassemble(&mut ram)?;

            let mut out = String::new();
            print_evt(&mut out, dis, PrintSettings::default())?;
            println!("{}", out);
            Ok(())
        }
        "network" => {
            if args.len() < 2 {
                return usage_fail()
            }

            let ip: IpAddr = args[0].parse().expect("Invalid IP");
            let port: u16 = args[1].parse().expect("Invalid port");
            let mut reader = ipcclient::IpcReader::new(ip, port).expect("Failed connection to server");

            let dis = disassemble(&mut reader).unwrap();
            let mut out = String::new();
            print_evt(&mut out, dis, PrintSettings::default()).unwrap();
            println!("{}", out);
            Ok(())
        }
        "help" => {
            usage();
            Ok(())
        }
        _ => {
            usage_fail()
        }
    }
}

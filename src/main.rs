use std::{env, io::Read};
use std::fs::File;
use std::{fmt, process};
use std::io::{self, Seek};

use reader::{FromReader, Endian, read_vec, DYNAMIC_SIZE};
use defs::Opcode;

pub mod reader;
pub mod defs;

pub fn check_float(val: i32) -> f32 {
    dbg!(val);
    dbg!(defs::EVTDAT_FLOAT_MAX);
    dbg!(val <= defs::EVTDAT_FLOAT_MAX);
    if val <= defs::EVTDAT_FLOAT_MAX {
        (val - defs::EVTDAT_FLOAT_BASE) as f32 / 1024.0
    }
    else {
        val as f32
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Address(pub u32);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:08x}", self.0)
    }
}

#[derive(Debug)]
pub enum EvtArg {
    GW(i32),
    GF(i32),
    GSW(i32),
    GSWF(i32),
    LW(i32),
    LF(i32),
    LSW(i32),
    LSWF(i32),
    UW(i32),
    UF(i32),
    ADDR(Address),
    INT(i32),
    FLOAT(f32)
}

impl EvtArg {
    pub fn decode(val: i32) -> EvtArg {
        if val <= defs::EVTDAT_ADDR_MAX {
            EvtArg::ADDR(Address(val as u32))
        }
        else if val <= defs::EVTDAT_FLOAT_MAX {
            EvtArg::FLOAT(check_float(val))
        }
        else if val <= defs::EVTDAT_UF_MAX {
            EvtArg::UF(val - defs::EVTDAT_UF_BASE)
        }
        else if val <= defs::EVTDAT_UW_MAX {
            EvtArg::UW(val - defs::EVTDAT_UW_BASE)
        }
        else if val <= defs::EVTDAT_GSW_MAX {
            EvtArg::GSW(val - defs::EVTDAT_GSW_BASE)
        }
        else if val <= defs::EVTDAT_LSW_MAX {
            EvtArg::LSW(val - defs::EVTDAT_LSW_BASE)
        }
        else if val <= defs::EVTDAT_GSWF_MAX {
            EvtArg::GSWF(val - defs::EVTDAT_GSWF_BASE)
        }
        else if val <= defs::EVTDAT_LSWF_MAX {
            EvtArg::LSWF(val - defs::EVTDAT_LSWF_BASE)
        }
        else if val <= defs::EVTDAT_GF_MAX {
            EvtArg::GF(val - defs::EVTDAT_GF_BASE)
        }
        else if val <= defs::EVTDAT_LF_MAX {
            EvtArg::LF(val - defs::EVTDAT_LF_BASE)
        }
        else if val <= defs::EVTDAT_GW_MAX {
            EvtArg::GW(val - defs::EVTDAT_GW_BASE)
        }
        else if val <= defs::EVTDAT_LW_MAX {
            EvtArg::LW(val - defs::EVTDAT_LW_BASE)
        }
        else {
            EvtArg::INT(val)
        }
    }
}

impl FromReader for EvtArg {
    type Args = ();

    const STATIC_SIZE: usize = i32::STATIC_SIZE;

    fn from_reader_args<R>(reader: &mut R, e: Endian, (): Self::Args) -> io::Result<Self>
    where R: Read + io::Seek + ?Sized {
        let int = i32::from_reader(reader, e)?;
        Ok(EvtArg::decode(int))
    }
}

#[derive(Debug)]
pub struct EvtInstr {
    pub opcode: Opcode,
    pub args: Vec<EvtArg>
}

impl FromReader for EvtInstr {
    type Args = (Address,);

    const STATIC_SIZE: usize = DYNAMIC_SIZE;

    fn from_reader_args<R>(reader: &mut R, e: Endian, _: Self::Args) -> io::Result<Self>
    where R: Read + io::Seek + ?Sized {
        let nargs = u16::from_reader(reader, e)? as usize;

        let opcode = u16::from_reader(reader, e)?;
        let opcode = Opcode::try_from(opcode)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid opcode"))?;

        let args = read_vec(reader, nargs, e)?;

        Ok(EvtInstr{ opcode, args})
    }
}

type EvtScript = Vec<EvtInstr>;

pub fn disasm_evt<R>(reader: &mut R) -> io::Result<EvtScript>
where R: Read + io::Seek + ?Sized {
    let mut ret = vec![];
    let mut opcode = Opcode::Next;
    while opcode != Opcode::EndScript {
        let instr = EvtInstr::from_reader(reader, Endian::Big)?;
        opcode = instr.opcode;
        ret.push(instr);
    }

    Ok(ret)
}

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

    dbg!(disasm_evt(&mut ram).expect("Couldn't read"));

    // https://github.com/encounter/decomp-toolkit/blob/18987ed330db864b48886b44a3d7fb222857e7e1/src/util/dol.rs#L147
}

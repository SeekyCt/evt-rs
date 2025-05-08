use crate::{Address, Endian, Arg, FromReader, Opcode, DYNAMIC_SIZE, read_vec};

use std::io;

#[derive(Debug)]
pub struct Instr {
    pub opcode: Opcode,
    pub args: Vec<Arg>,
}

impl FromReader for Instr {
    type Args = (Address,);

    const STATIC_SIZE: usize = DYNAMIC_SIZE;

    fn from_reader_args<R>(reader: &mut R, e: Endian, _: Self::Args) -> io::Result<Self>
    where
        R: io::Read + io::Seek + ?Sized,
    {
        let nargs = u16::from_reader(reader, e)? as usize;

        let opcode = u16::from_reader(reader, e)?;
        let opcode = Opcode::try_from(opcode)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid opcode"))?;

        let args = read_vec(reader, nargs, e)?;

        Ok(Instr { opcode, args })
    }
}

pub type Script = Vec<Instr>;

pub fn disassemble<R>(reader: &mut R) -> io::Result<Script>
where R: io::Read + io::Seek + ?Sized {
    let mut ret = vec![];
    let mut opcode = Opcode::Next;
    while opcode != Opcode::EndScript {
        let instr = Instr::from_reader(reader, Endian::Big)?;
        opcode = instr.opcode;
        ret.push(instr);
    }

    Ok(ret)
}

use crate::{Address, Endian, EvtArg, FromReader, Opcode, DYNAMIC_SIZE, read_vec};

use std::io;

#[derive(Debug)]
pub struct EvtInstr {
    pub opcode: Opcode,
    pub args: Vec<EvtArg>,
}

impl FromReader for EvtInstr {
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

        Ok(EvtInstr { opcode, args })
    }
}

type EvtScript = Vec<EvtInstr>;

pub fn disasm_evt<R>(reader: &mut R) -> io::Result<EvtScript>
where R: io::Read + io::Seek + ?Sized {
    let mut ret = vec![];
    let mut opcode = Opcode::Next;
    while opcode != Opcode::EndScript {
        let instr = EvtInstr::from_reader(reader, Endian::Big)?;
        opcode = instr.opcode;
        ret.push(instr);
    }

    Ok(ret)
}

use crate::{
    Address, Arg, Opcode,
    reader::{DYNAMIC_SIZE, Endian, FromReader, ToWriter, read_vec, write_vec},
};

use std::io;

#[derive(Debug, Clone, PartialEq)]
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

impl ToWriter for Instr {
    fn to_writer<W>(&self, writer: &mut W, e: Endian) -> io::Result<()>
    where
        W: io::Write + ?Sized,
    {
        let nargs = self.args.len() as u16;
        let opcode = self.opcode as u16;

        nargs.to_writer(writer, e)?;
        opcode.to_writer(writer, e)?;
        write_vec(writer, &self.args, e)?;

        Ok(())
    }

    fn write_size(&self) -> usize {
        u16::STATIC_SIZE * 2 + i32::STATIC_SIZE * self.args.len()
    }
}

pub type Script = Vec<Instr>;

pub fn disassemble<R>(reader: &mut R) -> io::Result<Script>
where
    R: io::Read + io::Seek + ?Sized,
{
    let mut ret = vec![];
    let mut opcode = Opcode::Next;
    while opcode != Opcode::EndScript {
        let instr = Instr::from_reader(reader, Endian::Big)?;
        opcode = instr.opcode;
        ret.push(instr);
    }

    Ok(ret)
}

pub fn assemble<W>(writer: &mut W, script: &Script) -> io::Result<()>
where
    W: io::Write + ?Sized,
{
    script.iter().try_for_each(|instr| instr.to_writer(writer, Endian::Big))
}

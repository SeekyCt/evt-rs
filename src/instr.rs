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
    script
        .iter()
        .try_for_each(|instr| instr.to_writer(writer, Endian::Big))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn read_instr<const N: usize>(bytes: [u8; N]) -> Instr {
        let mut cursor = Cursor::new(bytes);
        Instr::from_reader(&mut cursor, Endian::Big).unwrap()
    }

    fn write_instr(instr: Instr) -> Vec<u8> {
        let mut cursor = Cursor::new(vec![]);
        instr.to_writer(&mut cursor, Endian::Big).unwrap();
        cursor.into_inner()
    }

    fn read_disassemble<const N: usize>(bytes: [u8; N]) -> Script {
        let mut cursor = Cursor::new(bytes);
        disassemble(&mut cursor).unwrap()
    }

    fn write_assemble(script: Script) -> Vec<u8> {
        let mut cursor = Cursor::new(vec![]);
        assemble(&mut cursor, &script).unwrap();
        cursor.into_inner()
    }

    #[test]
    #[rustfmt::skip]
    fn test_reader() {
        assert_eq!(
            read_instr([
                0x00, 0x00, 0x00, 0x01,
            ]),
            Instr {opcode: Opcode::EndScript, args: vec![]}
        );

        assert_eq!(
            read_instr([
                0x00, 0x02, 0x00, 0x32,
                0xFE, 0x36, 0x3C, 0x80,
                0xF5, 0xDE, 0x01, 0x81,
            ]),
            Instr {opcode: Opcode::Set, args: vec![Arg::LW(0), Arg::GSW(1)]}
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_writer() {
        assert_eq!(
            write_instr(Instr {opcode: Opcode::EndScript, args: vec![]}),
            [
                0x00, 0x00, 0x00, 0x01,
            ]
        );

        assert_eq!(
            write_instr(Instr {opcode: Opcode::Set, args: vec![Arg::LW(0), Arg::GSW(1)]}),
            [
                0x00, 0x02, 0x00, 0x32,
                0xFE, 0x36, 0x3C, 0x80,
                0xF5, 0xDE, 0x01, 0x81,
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_disassemble() {
        assert_eq!(
            read_disassemble([
                0x00, 0x02, 0x00, 0x32,
                0xFE, 0x36, 0x3C, 0x80,
                0xF5, 0xDE, 0x01, 0x81,
                0x00, 0x00, 0x00, 0x01,
            ]),
            vec![
                Instr {opcode: Opcode::Set, args: vec![Arg::LW(0), Arg::GSW(1)]},
                Instr {opcode: Opcode::EndScript, args: vec![]},
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_assemble() {
        assert_eq!(
            write_assemble(vec![
                Instr {opcode: Opcode::Set, args: vec![Arg::LW(0), Arg::GSW(1)]},
                Instr {opcode: Opcode::EndScript, args: vec![]},
            ]),
            [
                0x00, 0x02, 0x00, 0x32,
                0xFE, 0x36, 0x3C, 0x80,
                0xF5, 0xDE, 0x01, 0x81,
                0x00, 0x00, 0x00, 0x01,
            ]
        );
    }
}

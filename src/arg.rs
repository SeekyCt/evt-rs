use crate::{Endian, FromReader, ToWriter};
use std::io;

use crate::Address;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arg {
    ADDR(Address),
    FLOAT(f32),
    UF(usize),
    UW(usize),
    GSW(usize),
    LSW(usize),
    GSWF(usize),
    LSWF(usize),
    GF(usize),
    LF(usize),
    GW(usize),
    LW(usize),
    INT(i32),
    NONE
}

pub const GSW_COUNT : usize = 2048;
pub const LSW_COUNT : usize = 1024;
pub const GSWF_COUNT : usize = 8192;
pub const LSWF_COUNT : usize = 512;
pub const GF_COUNT : usize = 96;
pub const LF_COUNT : usize = 96;
pub const GW_COUNT : usize = 32;
pub const LW_COUNT : usize = 16;

pub const ADDR_MAX: i32 = -290000000;
pub const FLOAT_MAX: i32 = -220000000;
pub const UF_MAX: i32 = -200000000;
pub const UW_MAX: i32 = -180000000;
pub const GSW_MAX: i32 = -160000000;
pub const LSW_MAX: i32 = -140000000;
pub const GSWF_MAX: i32 = -120000000;
pub const LSWF_MAX: i32 = -100000000;
pub const GF_MAX: i32 = -80000000;
pub const LF_MAX: i32 = -60000000;
pub const GW_MAX: i32 = -40000000;
pub const LW_MAX: i32 = -20000000;

pub const NONE: i32 = -270000000;
pub const FLOAT_BASE: i32 = -240000000;
pub const UF_BASE: i32 = -210000000;
pub const UW_BASE: i32 = -190000000;
pub const GSW_BASE: i32 = -170000000;
pub const LSW_BASE: i32 = -150000000;
pub const GSWF_BASE: i32 = -130000000;
pub const LSWF_BASE: i32 = -110000000;
pub const GF_BASE: i32 = -90000000;
pub const LF_BASE: i32 = -70000000;
pub const GW_BASE: i32 = -50000000;
pub const LW_BASE: i32 = -30000000;

impl Arg {
    pub fn decode(val: i32) -> Arg {
        if val == NONE {
            Arg::NONE
        }
        else if val <= ADDR_MAX {
            Arg::ADDR(Address(val as u32))
        }
        else if val <= FLOAT_MAX {
            Arg::FLOAT(check_float(val))
        }
        else if val <= UF_MAX {
            Arg::UF((val - UF_BASE) as usize)
        }
        else if val <= UW_MAX {
            Arg::UW((val - UW_BASE) as usize)
        }
        else if val <= GSW_MAX {
            Arg::GSW((val - GSW_BASE) as usize)
        }
        else if val <= LSW_MAX {
            Arg::LSW((val - LSW_BASE) as usize)
        }
        else if val <= GSWF_MAX {
            Arg::GSWF((val - GSWF_BASE) as usize)
        }
        else if val <= LSWF_MAX {
            Arg::LSWF((val - LSWF_BASE) as usize)
        }
        else if val <= GF_MAX {
            Arg::GF((val - GF_BASE) as usize)
        }
        else if val <= LF_MAX {
            Arg::LF((val - LF_BASE) as usize)
        }
        else if val <= GW_MAX {
            Arg::GW((val - GW_BASE) as usize)
        }
        else if val <= LW_MAX {
            Arg::LW((val - LW_BASE) as usize)
        }
        else {
            Arg::INT(val)
        }
    }

    pub fn encode(self) -> i32 {
        match self {
            Arg::NONE => NONE,
            Arg::ADDR(Address(addr)) => addr as i32,
            Arg::FLOAT(val) => change_float(val),
            Arg::UF(val) => val as i32 + UF_BASE,
            Arg::UW(val) => val as i32 + UW_BASE,
            Arg::GSW(val) => val as i32 + GSW_BASE,
            Arg::LSW(val) => val as i32 + LSW_BASE,
            Arg::GSWF(val) => val as i32 + GSWF_BASE,
            Arg::LSWF(val) => val as i32 + LSWF_BASE,
            Arg::GF(val) => val as i32 + GF_BASE,
            Arg::LF(val) => val as i32 + LF_BASE,
            Arg::GW(val) => val as i32 + GW_BASE,
            Arg::LW(val) => val as i32 + LW_BASE,
            Arg::INT(val) => val
        }
    }
}

impl FromReader for Arg {
    type Args = ();

    const STATIC_SIZE: usize = i32::STATIC_SIZE;

    fn from_reader_args<R>(reader: &mut R, e: Endian, (): Self::Args) -> io::Result<Self>
    where
        R: io::Read + io::Seek + ?Sized,
    {
        let int = i32::from_reader(reader, e)?;
        Ok(Arg::decode(int))
    }
}

impl ToWriter for Arg {
    fn to_writer<W>(&self, writer: &mut W, e: Endian) -> io::Result<()>
    where W: io::Write + ?Sized {
        self.encode().to_writer(writer, e)
    }
    
    fn write_size(&self) -> usize {
        Self::STATIC_SIZE
    }
}

pub fn check_float(val: i32) -> f32 {
    if val <= FLOAT_MAX {
        (val - FLOAT_BASE) as f32 / 1024.0
    }
    else {
        val as f32
    }
}

pub fn change_float(val: f32) -> i32
{
    (val * 1024.0) as i32 + FLOAT_BASE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_round_trip() {
        for i in 0x8000_0000..0x8180_0000 {
            let before = Arg::ADDR(Address(i));
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0x9000_0000..0x9400_0000 {
            let before = Arg::ADDR(Address(i));
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 1..10 {
            let before = Arg::FLOAT(i as f32);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..10 {
            let before = Arg::UF(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..10 {
            let before = Arg::UW(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..GSW_COUNT {
            let before = Arg::GSW(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..LSW_COUNT {
            let before = Arg::LSW(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..GSWF_COUNT {
            let before = Arg::GSWF(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..LSWF_COUNT {
            let before = Arg::LSWF(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..GF_COUNT {
            let before = Arg::GF(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..LF_COUNT {
            let before = Arg::LF(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..GW_COUNT {
            let before = Arg::GW(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in 0..LW_COUNT {
            let before = Arg::LW(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        for i in -10..10 {
            let before = Arg::INT(i);
            assert_eq!(before, Arg::decode(before.encode()));
        }

        let before = Arg::NONE;
        assert_eq!(before, Arg::decode(before.encode()));
    }
}

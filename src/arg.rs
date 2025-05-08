use crate::reader::{Endian, FromReader};
use std::io;

use crate::Address;

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
    FLOAT(f32),
}

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

pub const ADDR_BASE: i32 = -270000000;
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

impl EvtArg {
    pub fn decode(val: i32) -> EvtArg {
        if val <= ADDR_MAX {
            EvtArg::ADDR(Address(val as u32))
        }
        else if val <= FLOAT_MAX {
            EvtArg::FLOAT(check_float(val))
        }
        else if val <= UF_MAX {
            EvtArg::UF(val - UF_BASE)
        }
        else if val <= UW_MAX {
            EvtArg::UW(val - UW_BASE)
        }
        else if val <= GSW_MAX {
            EvtArg::GSW(val - GSW_BASE)
        }
        else if val <= LSW_MAX {
            EvtArg::LSW(val - LSW_BASE)
        }
        else if val <= GSWF_MAX {
            EvtArg::GSWF(val - GSWF_BASE)
        }
        else if val <= LSWF_MAX {
            EvtArg::LSWF(val - LSWF_BASE)
        }
        else if val <= GF_MAX {
            EvtArg::GF(val - GF_BASE)
        }
        else if val <= LF_MAX {
            EvtArg::LF(val - LF_BASE)
        }
        else if val <= GW_MAX {
            EvtArg::GW(val - GW_BASE)
        }
        else if val <= LW_MAX {
            EvtArg::LW(val - LW_BASE)
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
    where
        R: io::Read + io::Seek + ?Sized,
    {
        let int = i32::from_reader(reader, e)?;
        Ok(EvtArg::decode(int))
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

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

    fn to_signed(x: u32) -> i32 {
        let temp = x as i64; 
        if temp > 0x8000_0000 {
            return (temp - (0x8000_0000 * 2)) as i32;
        } else {
            return temp as i32;
        }
    }

    #[test]
    fn decode() {
        assert_eq!(Arg::decode(to_signed(0x8000_0000)), Arg::ADDR(Address(0x8000_0000)));
        assert_eq!(Arg::decode(to_signed(0x8000_0001)), Arg::ADDR(Address(0x8000_0001)));
        assert_eq!(Arg::decode(to_signed(0x9000_0000)), Arg::ADDR(Address(0x9000_0000)));
        assert_eq!(Arg::decode(to_signed(0x9000_0001)), Arg::ADDR(Address(0x9000_0001)));

        assert_eq!(Arg::decode(to_signed(0xf1b1e400)), Arg::FLOAT(0.0));
        assert_eq!(Arg::decode(to_signed(0xf1b1e800)), Arg::FLOAT(1.0));

        assert_eq!(Arg::decode(to_signed(0xf37ba780)), Arg::UF(0));
        assert_eq!(Arg::decode(to_signed(0xf37ba781)), Arg::UF(1));

        assert_eq!(Arg::decode(to_signed(0xf4acd480)), Arg::UW(0));
        assert_eq!(Arg::decode(to_signed(0xf4acd481)), Arg::UW(1));

        assert_eq!(Arg::decode(to_signed(0xf5de0180)), Arg::GSW(0));
        assert_eq!(Arg::decode(to_signed(0xf5de0181)), Arg::GSW(1));

        assert_eq!(Arg::decode(to_signed(0xf70f2e80)), Arg::LSW(0));
        assert_eq!(Arg::decode(to_signed(0xf70f2e81)), Arg::LSW(1));

        assert_eq!(Arg::decode(to_signed(0xf8405b80)), Arg::GSWF(0));
        assert_eq!(Arg::decode(to_signed(0xf8405b81)), Arg::GSWF(1));

        assert_eq!(Arg::decode(to_signed(0xf9718880)), Arg::LSWF(0));
        assert_eq!(Arg::decode(to_signed(0xf9718881)), Arg::LSWF(1));

        assert_eq!(Arg::decode(to_signed(0xfaa2b580)), Arg::GF(0));
        assert_eq!(Arg::decode(to_signed(0xfaa2b581)), Arg::GF(1));

        assert_eq!(Arg::decode(to_signed(0xfbd3e280)), Arg::LF(0));
        assert_eq!(Arg::decode(to_signed(0xfbd3e281)), Arg::LF(1));

        assert_eq!(Arg::decode(to_signed(0xfd050f80)), Arg::GW(0));
        assert_eq!(Arg::decode(to_signed(0xfd050f81)), Arg::GW(1));

        assert_eq!(Arg::decode(to_signed(0xfe363c80)), Arg::LW(0));
        assert_eq!(Arg::decode(to_signed(0xfe363c81)), Arg::LW(1));

        assert_eq!(Arg::decode(-1), Arg::INT(-1));
        assert_eq!(Arg::decode(0), Arg::INT(0));
        assert_eq!(Arg::decode(1), Arg::INT(1));

        assert_eq!(Arg::decode(to_signed(0xefe82080)), Arg::NONE);
    }

    #[test]
    fn encode() {
        assert_eq!(Arg::ADDR(Address(0x8000_0000)).encode(), to_signed(0x8000_0000));
        assert_eq!(Arg::ADDR(Address(0x8000_0001)).encode(), to_signed(0x8000_0001));
        assert_eq!(Arg::ADDR(Address(0x9000_0000)).encode(), to_signed(0x9000_0000));
        assert_eq!(Arg::ADDR(Address(0x9000_0001)).encode(), to_signed(0x9000_0001));

        assert_eq!(Arg::FLOAT(0.0).encode(), to_signed(0xf1b1e400));
        assert_eq!(Arg::FLOAT(1.0).encode(), to_signed(0xf1b1e800));

        assert_eq!(Arg::UF(0).encode(), to_signed(0xf37ba780));
        assert_eq!(Arg::UF(1).encode(), to_signed(0xf37ba781));

        assert_eq!(Arg::UW(0).encode(), to_signed(0xf4acd480));
        assert_eq!(Arg::UW(1).encode(), to_signed(0xf4acd481));

        assert_eq!(Arg::GSW(0).encode(), to_signed(0xf5de0180));
        assert_eq!(Arg::GSW(1).encode(), to_signed(0xf5de0181));

        assert_eq!(Arg::LSW(0).encode(), to_signed(0xf70f2e80));
        assert_eq!(Arg::LSW(1).encode(), to_signed(0xf70f2e81));

        assert_eq!(Arg::GSWF(0).encode(), to_signed(0xf8405b80));
        assert_eq!(Arg::GSWF(1).encode(), to_signed(0xf8405b81));

        assert_eq!(Arg::LSWF(0).encode(), to_signed(0xf9718880));
        assert_eq!(Arg::LSWF(1).encode(), to_signed(0xf9718881));

        assert_eq!(Arg::GF(0).encode(), to_signed(0xfaa2b580));
        assert_eq!(Arg::GF(1).encode(), to_signed(0xfaa2b581));

        assert_eq!(Arg::LF(0).encode(), to_signed(0xfbd3e280));
        assert_eq!(Arg::LF(1).encode(), to_signed(0xfbd3e281));

        assert_eq!(Arg::GW(0).encode(), to_signed(0xfd050f80));
        assert_eq!(Arg::GW(1).encode(), to_signed(0xfd050f81));

        assert_eq!(Arg::LW(0).encode(), to_signed(0xfe363c80));
        assert_eq!(Arg::LW(1).encode(), to_signed(0xfe363c81));

        assert_eq!(Arg::INT(-1).encode(), -1);
        assert_eq!(Arg::INT(0).encode(), 0);
        assert_eq!(Arg::INT(1).encode(), 1);

        assert_eq!(Arg::NONE.encode(), to_signed(0xefe82080));
    }

    #[test]
    fn round_trip() {
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

    // A decode->encode round trip is not guaranteed due to float rounding
}

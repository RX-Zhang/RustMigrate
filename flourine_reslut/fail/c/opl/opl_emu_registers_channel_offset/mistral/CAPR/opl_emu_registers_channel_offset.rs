

use libc::uint32_t;
use std::ops::*;

#[derive(Copy, Clone, Debug)]
struct Uint32(uint32_t);

impl Add for Uint32 {
    type Output = Uint32;
    fn add(self, other: Uint32) -> Uint32 {
        Uint32(self.0.wrapping_add(other.0))
    }
}

impl Mul for Uint32 {
    type Output = Uint32;
    fn mul(self, other: Uint32) -> Uint32 {
        Uint32(self.0.wrapping_mul(other.0))
    }
}

impl Div for Uint32 {
    type Output = Uint32;
    fn div(self, other: Uint32) -> Uint32 {
        Uint32(self.0.wrapping_div(other.0))
    }
}

impl Rem for Uint32 {
    type Output = Uint32;
    fn rem(self, other: Uint32) -> Uint32 {
        Uint32(self.0.wrapping_rem(other.0))
    }
}

fn opl_emu_registers_channel_offset(chnum: Uint32) -> Uint32 {
    let mut chnum = chnum;
    Uint32((chnum.0 % 9) + (100 * (chnum.0 / 9)))
}


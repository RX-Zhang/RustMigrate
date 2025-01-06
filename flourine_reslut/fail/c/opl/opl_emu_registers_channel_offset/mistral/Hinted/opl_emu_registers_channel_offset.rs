

use libc::uint32_t;
use std::ops::Add;

fn opl_emu_registers_channel_offset(chnum: uint32_t) -> uint32_t {
let nine = 9u32;
let quotient = chnum / nine;
let remainder = chnum % nine;
(remainder.wrapping_add((&nine * quotient) as u32))
}


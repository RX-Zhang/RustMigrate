

use libc::uint32_t;
use std::ops::BitAnd;
use std::ops::Shr;

fn opl_emu_bitfield(value: uint32_t, start: i32, length: u32) -> uint32_t {
    return (value.shr(start)) & ((1_u32).wrapping_shl(length) - 1);
}


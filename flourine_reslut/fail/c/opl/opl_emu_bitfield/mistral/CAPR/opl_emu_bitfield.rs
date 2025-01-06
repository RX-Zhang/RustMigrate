

use libc::uint32_t;
use std::primitive::u32; // use std::int is not available in Rust stable version, use std::primitive::u32 instead

fn opl_emu_bitfield(value: uint32_t, start: u32, length: u32) -> uint32_t {
    return (value >> start) & ((1u32.wrapping_shl(length)) - 1);
}


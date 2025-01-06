
use std::i32;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
let mask: u32 = (1u32 << (length as u32)) - 1;
let shifted_value = value.wrapping_shr((start % 32) as u32);
(shifted_value & mask) as u32
}

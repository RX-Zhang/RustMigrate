
use std::u32;
use std::i32;

fn opl_emu_field(value: u32, start: i32, length: i32) -> u32 {
    let shifted_value = value.wrapping_shr(start as u32);
    let mask = (1u32 << length) - 1;
    shifted_value & mask
}

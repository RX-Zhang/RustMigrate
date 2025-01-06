
use std::convert::TryInto;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let start: u32 = start.try_into().unwrap();
    let length: u32 = length.try_into().unwrap();
    ((value >> start) & ((1 << length) - 1))
}


use std::u32;

fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
let rem = chnum % 9;
let quot = (chnum / 9) as u32;
let quot_u32 = u32::wrapping_mul(0x100, quot);
rem + quot_u32
}


use std::u32;

fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    let quotient = (chnum as f32 / 9.0).floor() as u32;
    let remainder = chnum % 9;
    (remainder + quotient * 0x100) & 0xffffffff
}

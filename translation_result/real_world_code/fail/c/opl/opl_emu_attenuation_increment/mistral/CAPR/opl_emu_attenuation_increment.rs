
use std::u32;

fn opl_emu_bitfield(value: u32) -> u32 {
    let mut result = 0;
    if (value & 0x2000) != 0 {
        result |= 0x0100;
    }
    if (value & 0x1000) != 0 {
        result |= 0x0080;
    }
    if (value & 0x0800) != 0 {
        result |= 0x0004;
    }
    result
}

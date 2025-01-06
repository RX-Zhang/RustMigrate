
use std::cmp::min;

pub fn opl_emu_registers_effective_rate(rawrate: u32, ksr: u32) -> u32 {
    if rawrate == 0 {
        0
    } else {
        min(rawrate + ksr, 63)
    }
}

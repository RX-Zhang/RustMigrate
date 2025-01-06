
use std::u32;

const MIN: u32 = 0;
const MAX: u32 = 63;

fn opl_min(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

fn opl_emu_registers_effective_rate(rawrate: u32, ksr: u32) -> u32 {
    if rawrate == 0 {
        return 0;
    }
    opl_min(rawrate.wrapping_add(ksr), MAX)
}

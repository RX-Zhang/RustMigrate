
use std::u32;

fn opl_emu_registers_effective_rate(rawrate: u32, ksr: u32) -> u32 {
    return if rawrate == 0 {
        0
    } else {
        u32::wrapping_add(rawrate, ksr).min(63)
    };
}


use std::cmp::min;
use std::mem;

const OPL_MIN: u32 = 0;
const SIXTY_THREE: u32 = 63;

fn opl_emu_registers_effective_rate(rawrate: u32, ksr: u32) -> u32 {
    if rawrate == 0 {
        0
    } else {
        min(rawrate.wrapping_add(ksr), SIXTY_THREE)
    }
}

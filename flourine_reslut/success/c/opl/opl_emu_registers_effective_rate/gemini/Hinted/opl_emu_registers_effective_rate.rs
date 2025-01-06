
fn opl_emu_registers_effective_rate(rawrate: u32, ksr: u32) -> u32 {
    if rawrate == 0 {
        0
    } else {
        rawrate.wrapping_add(ksr).min(63)
    }
}

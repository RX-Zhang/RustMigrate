
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr(start as u32)) & ((1u32.wrapping_shl(length as u32)).wrapping_sub(1))
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    *noise_lfsr = (*noise_lfsr).wrapping_shl(1)
        | (opl_emu_bitfield(*noise_lfsr, 23, 1) ^ opl_emu_bitfield(*noise_lfsr, 9, 1)
            ^ opl_emu_bitfield(*noise_lfsr, 5, 1) ^ opl_emu_bitfield(*noise_lfsr, 0, 1));

    *lfo_am_counter = lfo_am_counter.wrapping_add(1);
    *lfo_pm_counter = lfo_pm_counter.wrapping_add(1);

    if *lfo_am_counter >= 64 {
        *lfo_am_counter = 0;
        *lfo_am = lfo_am.wrapping_add(1);
    }

    let noise = (*noise_lfsr & 1) as i32;
    let am = (((*lfo_am as u32) & 0x3f) * am_depth) >> 7;
    let pm = (((*lfo_pm_counter as u32) >> 10) * pm_depth) >> 5;

    (noise << 31) | ((am as i32) << 23) | ((pm as i32) << 7)
}

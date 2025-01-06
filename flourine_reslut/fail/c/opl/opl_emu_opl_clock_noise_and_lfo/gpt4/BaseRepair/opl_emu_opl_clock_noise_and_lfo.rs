
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_clock_noise_and_lfo(noise_lfsr: &mut u32, lfo_am_counter: &mut u16, lfo_pm_counter: &mut u16, lfo_am: &mut u8, am_depth: u32, pm_depth: u32) -> i32 {
    *noise_lfsr = noise_lfsr.wrapping_shl(1) | (opl_emu_bitfield(*noise_lfsr, 23, 1) ^ opl_emu_bitfield(*noise_lfsr, 9, 1) ^ opl_emu_bitfield(*noise_lfsr, 8, 1) ^ opl_emu_bitfield(*noise_lfsr, 1, 1));

    let mut am_counter = *lfo_am_counter as u32;
    am_counter = am_counter.wrapping_add(1);
    if am_counter >= 210 * 64 - 1 {
        *lfo_am_counter = 0;
    } else {
        *lfo_am_counter = am_counter as u16;
    }

    let shift = 9 - 2 * am_depth;
    *lfo_am = (((am_counter < 105 * 64) as u32 * am_counter + (am_counter >= 105 * 64) as u32 * (210 * 64 + 63 - am_counter)) >> shift) as u8;

    let mut pm_counter = *lfo_pm_counter as u32;
    pm_counter = pm_counter.wrapping_add(1);
    *lfo_pm_counter = pm_counter as u16;

    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    pm_scale[opl_emu_bitfield(pm_counter, 10, 3) as usize] as i32 >> (pm_depth ^ 1)
}

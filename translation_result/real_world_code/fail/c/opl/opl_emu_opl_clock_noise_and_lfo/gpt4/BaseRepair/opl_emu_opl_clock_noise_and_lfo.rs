
fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    *noise_lfsr <<= 1;
    *noise_lfsr |= opl_emu_bitfield(*noise_lfsr, 23, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 16, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 15, 1)
        ^ opl_emu_bitfield(*noise_lfsr, 1, 1);

    let am_counter: u32 = (*lfo_am_counter as u32).wrapping_add(1);
    *lfo_am_counter = if am_counter >= 210 * 64 {
        0
    } else {
        am_counter as u16
    };

    let angle = am_counter % (105 * 64);
    let angle = if am_counter < 105 * 64 {
        angle
    } else {
        210 * 64 - 1 - angle
    };

    *lfo_am = (angle >> (9 - 2 * am_depth)) as u8;

    let pm_counter: u32 = (*lfo_pm_counter as u32).wrapping_add(1);
    *lfo_pm_counter = pm_counter as u16;
    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    pm_scale[opl_emu_bitfield(pm_counter, 10, 3) as usize] as i32 >> (pm_depth & 0x01)
}

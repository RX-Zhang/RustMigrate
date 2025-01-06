
fn opl_emu_bitfield(value: u32, start: usize, length: usize) -> u32 {
    (value >> start) & ((1u32 << length).wrapping_sub(1))
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    *noise_lfsr = noise_lfsr.wrapping_shl(1) |
                  (opl_emu_bitfield(*noise_lfsr, 23, 1) ^
                   opl_emu_bitfield(*noise_lfsr, 9, 1) ^
                   opl_emu_bitfield(*noise_lfsr, 8, 1) ^
                   opl_emu_bitfield(*noise_lfsr, 1, 1));
    
    let am_counter_inc = lfo_am_counter.wrapping_add(1);
    *lfo_am_counter = if am_counter_inc >= 210 * 64 { 0 } else { am_counter_inc };

    let shift = (9 - 2 * am_depth) % 32;
    *lfo_am = if *lfo_am_counter < 105 * 64 {
        (*lfo_am_counter as u32 >> shift) as u8
    } else {
        ((210 * 64 - 1 - *lfo_am_counter as u32) >> shift) as u8
    };

    let pm_counter_inc = lfo_pm_counter.wrapping_add(1);
    *lfo_pm_counter = pm_counter_inc;

    let pm_index = opl_emu_bitfield(*lfo_pm_counter as u32, 10, 3) as usize;
    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    pm_scale[pm_index] as i32 >> ((pm_depth & 7) as i32 ^ 1)
}

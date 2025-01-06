
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let start = start as u32;
    let length = length as u32;
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
    let feedback = ((*noise_lfsr >> 23) & 1)
        ^ ((*noise_lfsr >> 9) & 1)
        ^ ((*noise_lfsr >> 8) & 1)
        ^ ((*noise_lfsr >> 1) & 1);
    *noise_lfsr = (*noise_lfsr << 1) | (feedback as u32);

    let am_counter = *lfo_am_counter;
    *lfo_am_counter = lfo_am_counter.wrapping_add(1);
    if am_counter >= 210 * 64 - 1 {
        *lfo_am_counter = 0;
    }

    let shift = 9 - 2 * am_depth as i32;
    if shift < 0 {
        *lfo_am = 0; // Handle negative shift
    } else {
        *lfo_am = if am_counter < 105 * 64 {
            ((am_counter as u32) >> shift as u32) as u8
        } else {
            (((210 * 64 + 63 - am_counter as u32) >> shift as u32) as u8)
        };
    }

    let pm_counter = *lfo_pm_counter;
    *lfo_pm_counter = lfo_pm_counter.wrapping_add(1);

    let pm_scale: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    pm_scale[(pm_counter % 8) as usize] as i32 >> (pm_depth % 2)
}


use std::ops::{Shl, Shr};

fn opl_emubitfieldvalue(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_lfsr_shift(noise_lfsr: &mut u32) -> u32 {
    let mut lfsr = *noise_lfsr;
    lfsr <<= 1;
    lfsr |= (lfsr >> 23) ^ (lfsr >> 9) & 1 ^ (lfsr >> 8) & 1 ^ (lfsr >> 1) & 1;
    *noise_lfsr = lfsr;
    lfsr & 1
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    let am_counter = *lfo_am_counter;
    *lfo_am_counter = am_counter.wrapping_add(1);
    let shift = 9 - 2 * am_depth as i32;
    *lfo_am = if am_counter < 105 * 64 {
        (am_counter >> shift) as u8
    } else {
        ((210 * 64 + 63 - am_counter) >> shift) as u8
    };
    let pm_counter = *lfo_pm_counter;
    *lfo_pm_counter = pm_counter.wrapping_add(1);
    let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
    let index = (pm_counter >> 10) & 7;
    (pm_scale[index as usize] >> (pm_depth ^ 1)) as i32 *
        if opl_emu_lfsr_shift(noise_lfsr) == 1 {
            1
        } else {
            -1
        }
}

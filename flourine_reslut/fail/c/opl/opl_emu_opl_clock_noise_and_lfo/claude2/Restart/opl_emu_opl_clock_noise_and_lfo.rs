

use std::boxed::Box;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut Box<u32>,
    lfo_am_counter: &mut Box<u16>,
    lfo_pm_counter: &mut Box<u16>,
    lfo_am: &mut Box<u8>,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {

    **noise_lfsr = noise_lfsr.wrapping_shl(1);
    **noise_lfsr |= opl_emu_bitfield(**noise_lfsr, 23, 1) ^
                  opl_emu_bitfield(**noise_lfsr, 9, 1) ^
                  opl_emu_bitfield(**noise_lfsr, 8, 1) ^
                  opl_emu_bitfield(**noise_lfsr, 1, 1);

    let am_counter = **lfo_am_counter as u32;
    if am_counter >= (210 * 64 - 1) {
        **lfo_am_counter = 0;
    }

    let shift = 9 - 2 * am_depth;

    *lfo_am = if am_counter < (105 * 64) {
        Box::new((am_counter >> shift) as u8)
    } else {
        Box::new(((210 * 64 + 63 - am_counter) >> shift) as u8)
    };

    let pm_counter = **lfo_pm_counter as u32;
    static PM_SCALE: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    (PM_SCALE[(pm_counter >> 10) as usize] >> (pm_depth ^ 1)).into()
}


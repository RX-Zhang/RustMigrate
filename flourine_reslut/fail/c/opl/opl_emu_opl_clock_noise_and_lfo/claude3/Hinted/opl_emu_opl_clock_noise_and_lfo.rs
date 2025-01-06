
use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    ((value >> start) & ((1 << length) - 1))
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut Box<u32>,
    lfo_am_counter: &mut Box<u16>,
    lfo_pm_counter: &mut Box<u16>,
    lfo_am: &mut Box<u8>,
    am_depth: u32,
    pm_depth: u32,
) -> i32 {
    let noise_lfsr_val = **noise_lfsr;
    *noise_lfsr = Box::new(
        (Wrapping(noise_lfsr_val) << 1)
            .0
            .wrapping_add(
                opl_emu_bitfield(noise_lfsr_val, 23, 1)
                    ^ opl_emu_bitfield(noise_lfsr_val, 9, 1)
                    ^ opl_emu_bitfield(noise_lfsr_val, 8, 1)
                    ^ opl_emu_bitfield(noise_lfsr_val, 1, 1),
            ),
    );

    let mut am_counter = **lfo_am_counter;
    *lfo_am_counter = Box::new(am_counter.wrapping_add(1));
    if **lfo_am_counter > 210 * 64 - 2 {
        *lfo_am_counter = Box::new(0);
    }

    let shift = 9 - 2 * am_depth as i32;
    let am_value = if **lfo_am_counter < 105 * 64 {
        **lfo_am_counter
    } else {
        210 * 64 + 63 - **lfo_am_counter
    } >> shift;
    *lfo_am = Box::new(am_value as u8);

    let mut pm_counter = **lfo_pm_counter;
    *lfo_pm_counter = Box::new(pm_counter.wrapping_add(1));

    let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
    let pm_value = pm_scale[opl_emu_bitfield(pm_counter.wrapping_add(0) as u32, 10, 3) as usize]
        >> (pm_depth ^ 1);
    pm_value as i32
}

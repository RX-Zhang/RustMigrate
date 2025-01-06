
use std::mem;

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
    let noise_lfsr_value = **noise_lfsr;
    let lfo_am_counter_value = **lfo_am_counter;
    let lfo_pm_counter_value = **lfo_pm_counter;
    let mut lfo_am_value = **lfo_am;

    *noise_lfsr = Box::new(
        noise_lfsr_value.wrapping_shl(1)
            | opl_emu_bitfield(noise_lfsr_value, 23, 1)
                ^ opl_emu_bitfield(noise_lfsr_value, 9, 1)
                ^ opl_emu_bitfield(noise_lfsr_value, 8, 1)
                ^ opl_emu_bitfield(noise_lfsr_value, 1, 1),
    );

    let mut am_counter = lfo_am_counter_value.wrapping_add(1);
    if am_counter >= 210 * 64 - 1 {
        am_counter = 0;
    }
    *lfo_am_counter = Box::new(am_counter);

    let shift = 9 - 2 * am_depth;

    lfo_am_value = if am_counter < 105 * 64 {
        (am_counter >> shift) as u8
    } else {
        ((210 * 64 + 63 - am_counter) >> shift) as u8
    };
    *lfo_am = Box::new(lfo_am_value);

    let mut pm_counter = lfo_pm_counter_value.wrapping_add(1);

    static PM_SCALE: [i8; 8] = [8, 4, 0, -4, -8, -4, 0, 4];
    let pm_scale_index = opl_emu_bitfield(pm_counter as u32, 10, 3) as usize;
    *lfo_pm_counter = Box::new(pm_counter);

    (PM_SCALE[pm_scale_index] >> (pm_depth ^ 1)) as i32
}


use std::num::Wrapping;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    ((value >> start) & ((1 << length) - 1)) as u32
}

fn opl_emu_opl_compute_phase_step(
    block_freq: u32,
    multiple: u32,
    lfo_raw_pm: i32,
) -> u32 {
    let fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;

    let mut fnum_wrapping = Wrapping(fnum as i32);
    let fnum_add = Wrapping((lfo_raw_pm * opl_emu_bitfield(block_freq, 7, 3) as i32) >> 1);
    fnum_wrapping = Wrapping(fnum_wrapping.0.wrapping_add(fnum_add.0));

    let fnum = fnum_wrapping.0 as u32 & 0xfff;

    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (fnum << block) >> 2;

    (phase_step * multiple) >> 1
}

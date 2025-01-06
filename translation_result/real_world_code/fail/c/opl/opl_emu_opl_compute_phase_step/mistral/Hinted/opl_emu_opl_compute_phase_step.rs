

use std::ops::Shr;
use std::ops::Shl;
use std::usize;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let fnum = opl_emu_bitfield(block_freq, 0, 10) << 2;
    let lfo_pm_shifted = ((lfo_raw_pm as i32) * (opl_emu_bitfield(block_freq, 7, 3) as i32)) as u32;
    let lfo_pm_shifted_u32 = lfo_pm_shifted as u32;
    let fnum = (fnum.wrapping_add(lfo_pm_shifted_u32)) & 0xfff;
    let block = opl_emu_bitfield(block_freq, 10, 3) as u32;
    let phase_step = (fnum << block) >> 2;
    (phase_step * multiple) >> 1
}


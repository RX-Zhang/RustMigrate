

use std::i32;
use std::primitive::u32;
use std::ops::{Shr, Shl, BitAnd};

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> (start as u32)) & (((1 as u32) << (length as u32)) - 1)
}

fn opl_emu_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    let fnum = ((opl_emu_bitfield(block_freq, 0, 10) << 2)
        .wrapping_add((((lfo_raw_pm & 0xFFFF) as u32) * opl_emu_bitfield(block_freq, 7, 3)) >> 1)) & 0xfff;
    let block = opl_emu_bitfield(block_freq, 10, 3);
    let phase_step = (((fnum << block) >> 2) * multiple) >> 1;
    phase_step
}


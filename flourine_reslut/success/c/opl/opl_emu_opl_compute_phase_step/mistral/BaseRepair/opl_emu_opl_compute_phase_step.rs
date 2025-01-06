

use std::i32;
use std::u32;
use libc::uint32_t;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1u32 << length) - 1)
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    // extract frequency number as a 12-bit fraction
    let fnum = (block_freq & 0x3ff) << 2;

    // apply the phase adjustment based on the upper 3 bits
    // of FNUM and the PM depth parameters
    let lfo_multiplier = ((block_freq >> 7) & 0x7) as i32;
    let lfo_scaled = (lfo_raw_pm as u32).wrapping_mul(lfo_multiplier as u32);
    let lfo_shifted = lfo_scaled >> 1;
    let fnum = fnum.wrapping_add(lfo_shifted as u32);

    // keep fnum to 12 bits
    let fnum = fnum & 0xfff;

    // apply block shift to compute phase step
    let block = (block_freq >> 10) & 0x7;
    let phase_step = (fnum << block) >> 2;

    // apply frequency multiplier (which is cached as an x.1 value)
    (phase_step * multiple) >> 1
}


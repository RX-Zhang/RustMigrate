

use std::i32;
use std::u32;
use std::boxed::Box;
use libc::uint32_t;

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_opl_compute_phase_step(block_freq: u32, multiple: u32, lfo_raw_pm: i32) -> u32 {
    // extract frequency number as a 12-bit fraction
    let fnum = (block_freq & 0x3ff) << 2;

    // apply the phase adjustment based on the upper 3 bits
    // of FNUM and the PM depth parameters
    let block_freq_shifted = (block_freq >> 7) & 0x7;
    let lfo_raw_pm_scaled = lfo_raw_pm as u32 * block_freq_shifted;
    let lfo_raw_pm_shifted = lfo_raw_pm_scaled >> 1;
    let fnum_adj = fnum.wrapping_add(lfo_raw_pm_shifted);

    // keep fnum to 12 bits
    let fnum_adj = fnum_adj & 0xfff;

    // apply block shift to compute phase step
    let block = (block_freq >> 10) & 0x7;
    let phase_step = (fnum_adj << block) >> 2;

    // apply frequency multiplier (which is cached as an x.1 value)
    (phase_step.wrapping_mul(multiple)) >> 1
}


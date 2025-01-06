

use std::mem;

const LFO_AM_STEPS: u32 = 210 * 64;
const LFO_PM_STEPS: u32 = 8192;
const LFO_PM_CHUNKS: usize = 8;

fn opl_emu_bitfield(value: u32) -> u32 {
let mut result = 0;
for i in 0..8 {
let mask = 1 << i;
if (value & (1 << i)) != 0 {
result |= mask;
}
}
result
}

fn lfo_pm_calc(mut lfo_pm_state: u32, steps: u32) -> u32 {
let lfo_pm_inc = 1u32.wrapping_add(u32::wrapping_mul(2, steps));
if (lfo_pm_inc & 1) == 0 {
lfo_pm_inc.wrapping_add(1);
}
lfo_pm_state = lfo_pm_state.wrapping_add(lfo_pm_inc);
let lfo_pm_val = opl_emu_bitfield(lfo_pm_state >> 14);
lfo_pm_val
}

fn lfo_pm_chunks(lfo_pm_state: u32, chunks: usize) -> Vec<u32> {
let mut result = Vec::new();
for _ in 0..chunks {
result.push(lfo_pm_calc(lfo_pm_state, LFO_PM_STEPS / LFO_PM_CHUNKS as u32));
}
result
}


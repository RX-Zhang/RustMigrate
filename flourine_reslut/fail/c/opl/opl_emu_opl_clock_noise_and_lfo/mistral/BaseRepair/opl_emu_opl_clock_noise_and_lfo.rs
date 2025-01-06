
use std::mem;

const LFO_AM_STEPS: u32 = 210 * 64;
const LFO_PM_STEPS: u32 = 8192;
const LFO_PM_CHUNKS: usize = 8;

fn opl_emu_bitfield(value: u32) -> u32 {
value & 0x0000FF00 |
(value & 0x000000FF) << 24 |
(value & 0x00FF0000) >> 16
}

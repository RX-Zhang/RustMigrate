

use std::mem;

const LFO_AM_STEPS: u32 = (210 * 64) & 0xffffffff;
const LFO_PM_STEPS: u32 = 8192;
const LFO_PM_CHUNKS: usize = 8;

fn opl_emu_bitfield(value: u32) -> u32 {
let mut result = 0;
for i in 0..8 {
let mask = 1 << (7 - i);
if (value & (1 << i)) != 0 {
result |= mask;
}
if (value & (1 << (15 - i))) != 0 {
result |= mask << 8;
}
}
result
}


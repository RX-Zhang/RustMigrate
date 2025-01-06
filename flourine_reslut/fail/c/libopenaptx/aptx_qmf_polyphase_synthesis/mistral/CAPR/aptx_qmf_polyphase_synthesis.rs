

use std::mem;

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
buffer: Box<[i32; 2 * FILTER_TAPS]>,
pos: u8,
}

fn clip_intp2(a: i64, p: u32) -> i32 {
if ((a as u64).wrapping_add(1 << p)) & !((2 << p) - 1) != 0 {
return (((a >> 63) & 1) as i32) ^ ((1 << p) - 1);
} else {
return (a & (i64::MAX as i64)) as i32;
}
}


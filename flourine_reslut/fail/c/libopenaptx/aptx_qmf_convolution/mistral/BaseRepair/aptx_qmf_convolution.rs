

use libc::{uint8_t, int64_t, c_int};
use std::boxed;
use std::mem;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[c_int; 2 * FILTER_TAPS]>,
    pos: uint8_t,
}

fn clip_intp2(a: c_int, p: u32) -> c_int {
    if ((a as u32).wrapping_add(1 << p) & !((1 << (p + 1)) - 1)) != 0 {
        return ((a >> 31) & 1) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding = (1 << (shift - 1)) as int64_t;
    (value >> shift) + rounding
}


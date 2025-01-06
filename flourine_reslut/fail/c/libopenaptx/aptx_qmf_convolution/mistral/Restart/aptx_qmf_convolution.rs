

use libc::int32_t;
use libc::int64_t;
use std::mem;
use std::boxed;
use std::usize;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
buffer: Box<[int32_t; 2 * FILTER_TAPS]>,
pos: usize,
}

fn clip_intp2(a: int32_t, p: usize) -> int32_t {
if ((a as usize) + (1 << p)) & !((2 << p) - 1) != 0 {
return ((a >> 31) as int32_t) ^ ((1 << p) - 1);
} else {
return a;
}
}

fn rshift64(value: int64_t, shift: usize) -> int64_t {
let rounding = (1 << (shift - 1)) as int64_t;
let mask = ((1 << (shift + 1)) - 1) as int64_t;
let masked_value = value & mask;
let shifted_value = (value + rounding) >> shift;
let result = shifted_value - (if masked_value == rounding {1} else {0});
return result;
}

fn rshift64_clip24(value: int64_t, shift: usize) -> int32_t {
clip_intp2((value as int32_t), 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[int32_t; FILTER_TAPS], shift: usize) -> int32_t {
let sig = &signal.buffer[signal.pos..];
let mut e: int64_t = 0;
let mut i: usize = 0;

for i in 0..FILTER_TAPS {
e = e.wrapping_add((sig[i] as int64_t) * (coeffs[i] as int64_t));
}

return rshift64_clip24(e, shift);
}


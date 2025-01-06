

use libc::int32_t;
use libc::int64_t;
use std::mem;
use std::option::Option;
use std::ptr;

const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [int32_t; 2 * FILTER_TAPS],
    pos: u8,
}

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        return ((a >> 31) & !((1 << p) - 1)) as int32_t;
    } else {
        return a;
    }
}

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding = (1 << (shift - 1)) as int64_t;
    let mask = ((1 << (shift + 1)) - 1) as int64_t;
    return ((value + rounding) >> shift) - ((value & mask) == rounding) as int64_t;
}

fn rshift64_clip24(value: int64_t, shift: u32) -> int32_t {
    return clip_intp2(rshift64(value, shift) as int32_t, 23);
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let sig = &signal.buffer[..];
    let mut e: i64 = 0;
    let mut i: usize = 0;

    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[signal.pos as usize + i] as i64) * (coeffs[i] as i64));
    }

    return rshift64_clip24(e, shift) as i32;
}


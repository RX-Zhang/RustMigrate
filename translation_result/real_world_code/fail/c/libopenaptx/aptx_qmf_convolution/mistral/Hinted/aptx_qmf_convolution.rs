
use libc::int32_t;
use std::mem;
use std::ops::Shr;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: usize,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    let shifted = ((value + rounding) >> shift) & !(mask >> shift);
    return shifted;
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    return clip_intp2(rshift64(value, shift) as i32, 23);
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos..];
    let mut e: i64 = 0;
    let coeffs_ptr: &[i32] = coeffs;

    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[i] as i64) * (coeffs_ptr[i] as i64));
    }

    return rshift64_clip24(e, shift);
}

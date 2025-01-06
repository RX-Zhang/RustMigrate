
use std::u32;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: usize,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1 << p) - 1;
    let shifted_a = a >> 31;
    let shifted_mask = mask as i32;
    (shifted_a & 1) ^ (-(a & mask as i32) as i32 & shifted_mask)
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    (value + rounding) >> shift & !(mask - rounding)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos..];
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        e += (sig[i] as i64) * (coeffs[i] as i64);
    }

    rshift64_clip24(e, shift)
}

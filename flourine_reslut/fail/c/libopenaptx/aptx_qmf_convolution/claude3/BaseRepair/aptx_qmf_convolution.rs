
use std::ops::{BitAnd, BitOr, Shl, Shr};

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: usize,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1u32 << (p + 1)) - 1;
    let a = a as u32;
    if a.wrapping_add(1u32 << p) & !mask != 0 {
        (((a as i32) >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a as i32
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos..];
    let mut e = 0i64;
    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[i] as i64) * (coeffs[i] as i64));
    }
    rshift64_clip24(e, shift)
}

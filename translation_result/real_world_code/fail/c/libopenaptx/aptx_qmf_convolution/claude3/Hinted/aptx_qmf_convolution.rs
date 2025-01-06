
use std::num::Wrapping;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (Wrapping(a as u32) + Wrapping(1u32 << p)).0 & !(Wrapping(2u32 << p) - Wrapping(1u32)).0 != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        let sig_index = (signal.pos as usize + i) % (2 * FILTER_TAPS);
        e += i64::from(signal.buffer[sig_index]) * i64::from(coeffs[i]);
    }

    rshift64_clip24(e, shift)
}

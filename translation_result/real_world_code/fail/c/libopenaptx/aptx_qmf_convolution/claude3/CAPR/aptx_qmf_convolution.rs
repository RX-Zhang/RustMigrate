
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
    let rounding = 1i64 << (shift.wrapping_sub(1));
    let mask = (1i64 << (shift.wrapping_add(1) % 64)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> (shift % 64)).wrapping_sub((value & mask == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let mut e = 0i64;
    for i in 0..FILTER_TAPS {
        let sig_idx = signal.pos.wrapping_add(i as u8) as usize % (2 * FILTER_TAPS);
        e = e.wrapping_add((signal.buffer[sig_idx] as i64).wrapping_mul(coeffs[i] as i64));
    }
    rshift64_clip24(e, shift)
}

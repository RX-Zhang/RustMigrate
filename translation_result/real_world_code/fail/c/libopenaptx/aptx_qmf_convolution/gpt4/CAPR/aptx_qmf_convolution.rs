
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p) & !((2u32.wrapping_shl(p)).wrapping_sub(1))) != 0 {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64.wrapping_shl(shift.wrapping_sub(1));
    let mask = (1i64.wrapping_shl(shift + 1)).wrapping_sub(1);
    value.wrapping_add(rounding).wrapping_shr(shift)
        - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(value.wrapping_shr(shift as u32) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let signal_index = (signal.pos & (2 * FILTER_TAPS - 1) as u8) as usize;
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        let sig_idx = (signal_index + i) & (2 * FILTER_TAPS - 1);
        e = e.wrapping_add(signal.buffer[sig_idx] as i64 * coeffs[i] as i64);
    }

    rshift64_clip24(e, shift)
}

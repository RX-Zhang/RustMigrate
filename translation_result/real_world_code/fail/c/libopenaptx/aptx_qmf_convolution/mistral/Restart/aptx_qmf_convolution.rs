
const FILTER_TAPS: usize = 16;

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: usize,
}

fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as u32).wrapping_add((1 << p) as u32) & !(2 << p - 1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    (value.wrapping_add(rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: usize) -> i32 {
    clip_intp2((rshift64(value, shift)) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: usize) -> i32 {
    let sig = &signal.buffer[signal.pos..(signal.pos + FILTER_TAPS)];
    let mut e: i64 = 0;

    for (i, &coeff) in coeffs.iter().enumerate() {
        e += (sig[i] as i64) * (coeff as i64);
    }

    rshift64_clip24(e, shift)
}

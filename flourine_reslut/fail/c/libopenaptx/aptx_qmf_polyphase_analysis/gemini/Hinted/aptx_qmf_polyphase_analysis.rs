
use std::ops::Add;

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 2],
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32 + (1 << p)) & !(((2 << p) - 1) as u32)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    ((value + rounding) >> shift) & mask
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; 2], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e: i64 = 0;
    for i in 0..2 {
        e += (sig[i] as i64) * (coeffs[i] as i64);
    }
    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos + 2) as usize] = sample;
    signal.pos = (signal.pos + 1) & (2 - 1);
}

fn aptx_qmf_polyphase_analysis(
    signal: &mut [AptxFilterSignal; 2],
    coeffs: &[[i32; 2]; 2],
    shift: u32,
    samples: &[i32; 2],
    low_subband_output: &mut i32,
    high_subband_output: &mut i32,
) {
    let mut subbands = [0; 2];
    for i in 0..2 {
        aptx_qmf_filter_signal_push(&mut signal[i], samples[1 - i]);
        subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
    *low_subband_output = clip_intp2(subbands[0].wrapping_add(subbands[1]), 23);
    *high_subband_output = clip_intp2(subbands[0].wrapping_sub(subbands[1]), 23);
}

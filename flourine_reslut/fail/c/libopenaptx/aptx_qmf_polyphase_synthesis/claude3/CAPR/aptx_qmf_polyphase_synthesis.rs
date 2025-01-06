
use std::ops::{BitAnd, BitOr, Shr};

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1u32 << p) - 1;
    let max_val = (1i32 << (p - 1)) - 1;
    let min_val = -(1i32 << (p - 1));

    if a > max_val {
        max_val
    } else if a < min_val {
        min_val
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    let result = ((value + rounding) >> shift) - ((value & mask) == rounding) as i64;
    result
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2((rshift64(value, shift) as i32), 23)
}

fn aptx_qmf_convolution(
    signal: &AptxFilterSignal,
    coeffs: &[i32; FILTER_TAPS],
    shift: u32,
) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        e = e.wrapping_add((sig[i] as i64) * (coeffs[i] as i64));
    }

    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize + FILTER_TAPS) & (2 * FILTER_TAPS - 1)] = sample;
    signal.pos = signal.pos.wrapping_add(1) & (FILTER_TAPS as u8 - 1);
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [AptxFilterSignal; NB_FILTERS],
    coeffs: &[[i32; FILTER_TAPS]; NB_FILTERS],
    shift: u32,
    low_subband_input: i32,
    high_subband_input: i32,
    samples: &mut [i32; NB_FILTERS],
) {
    let mut subbands = [0i32; NB_FILTERS];

    subbands[0] = low_subband_input.wrapping_add(high_subband_input);
    subbands[1] = low_subband_input.wrapping_sub(high_subband_input);

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], subbands[1 - i]);
        samples[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
}

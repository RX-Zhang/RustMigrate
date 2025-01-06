

use std::mem;

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: usize,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1u32 << p) - 1;
    if a.wrapping_add(1i32.wrapping_shl(p)).wrapping_add((!(mask << 1)).wrapping_shr(32).try_into().unwrap()) != 0 {
        (a >> 31) ^ (mask as i32)
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
    clip_intp2((rshift64(value, shift) as i32), 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos..];
    let mut e = 0i64;
    for i in 0..FILTER_TAPS {
        e += sig[i] as i64 * coeffs[i] as i64;
    }
    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos] = sample;
    signal.buffer[signal.pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos + 1) & (FILTER_TAPS - 1);
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [AptxFilterSignal; NB_FILTERS],
    coeffs: &[Box<[i32; FILTER_TAPS]>; NB_FILTERS],
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



use std::convert::TryInto;

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a.wrapping_add(1 << p) & !((2 << p) - 1)) != 0 {
        (!(a >> 31)) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(value.try_into().unwrap(), 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..][..FILTER_TAPS];
    let mut e: i64 = 0;
    for i in 0..FILTER_TAPS {
        e += (sig[i] as i64) * (coeffs[i] as i64);
    }
    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize + FILTER_TAPS) % signal.buffer.len()] = sample;
    signal.pos = signal.pos.wrapping_add(1) & (FILTER_TAPS - 1) as u8;
}

fn aptx_qmf_polyphase_synthesis(
    mut signals: [AptxFilterSignal; NB_FILTERS],
    coeffs: [[i32; FILTER_TAPS]; NB_FILTERS],
    shift: u32,
    low_subband_input: i32,
    high_subband_input: i32,
    samples: &mut [i32; NB_FILTERS],
) {
    let mut subbands = [0; NB_FILTERS];
    subbands[0] = low_subband_input + high_subband_input;
    subbands[1] = low_subband_input - high_subband_input;

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signals[1 - i], subbands[i]);
        samples[i] = aptx_qmf_convolution(&signals[i], &coeffs[i], shift);
    }
}



const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: usize,
}

fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as u32) + (1 << p)) & !(((2 << p) - 1) as u32) != 0 {
        (a >> 31) ^ ((1 << p) - 1) as i32
    } else {
        a
    }
}

fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding as i64) as i64
}

fn rshift64_clip24(value: i64, shift: usize) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: usize) -> i32 {
    let sig = &signal.buffer[signal.pos..];
    let mut e: i64 = 0;

    for i in 0..FILTER_TAPS {
        e += sig[i] as i64 * coeffs[i] as i64;
    }

    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos] = sample;
    signal.buffer[signal.pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos + 1) % FILTER_TAPS;
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [AptxFilterSignal; NB_FILTERS],
    coeffs: &[[i32; FILTER_TAPS]; NB_FILTERS],
    shift: usize,
    low_subband_input: i32,
    high_subband_input: i32,
    samples: &mut [i32; NB_FILTERS],
) {
    let subbands = [
        low_subband_input + high_subband_input,
        low_subband_input - high_subband_input,
    ];

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], subbands[1 - i]);
        samples[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
}

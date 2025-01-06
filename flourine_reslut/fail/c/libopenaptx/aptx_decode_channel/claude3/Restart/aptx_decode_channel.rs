

use std::ops::{BitAnd, BitOr, Shr};

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

static APTX_QMF_INNER_COEFFS: [[i32; FILTER_TAPS]; NB_FILTERS] = [
    [
        1033, -584, -13592, 61697, -171156, 381799, -828088, 3962579, 985888, -226954, 39048,
        11990, -14203, 4966, 973, -1268,
    ],
    [
        -1268, 973, 4966, -14203, 11990, 39048, -226954, 985888, 3962579, -828088, 381799,
        -171156, 61697, -13592, -584, 1033,
    ],
];
static APTX_QMF_OUTER_COEFFS: [[i32; FILTER_TAPS]; NB_FILTERS] = [
    [
        730, -413, -9611, 43626, -121026, 269973, -585547, 2801966, 697128, -160481, 27611, 8478,
        -10043, 3511, 688, -897,
    ],
    [
        -897, 688, 3511, -10043, 8478, 27611, -160481, 697128, 2801966, -585547, 269973, -121026,
        43626, -9611, -413, 730,
    ],
];

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

struct AptxPrediction {
    prev_sign: Box<[i32; 2]>,
    s_weight: Box<[i32; 2]>,
    d_weight: Box<[i32; 24]>,
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: Box<[AptxFilterSignal; NB_FILTERS]>,
    inner_filter_signal: Box<[Box<[AptxFilterSignal; NB_FILTERS]>; NB_FILTERS]>,
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: Box<[i32; NB_SUBBANDS]>,
    qmf: AptxQMFAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvertQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = ((1u32 << p) - 1) as i32;
    if (a.wrapping_add(1 << p) & !(((2u32 << p) - 1) as i32)) != 0 {
        (a >> 31) ^ mask
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = ((1i64 << (shift + 1)) - 1) as i64;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2((rshift64(value, shift) as i32), 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let mut e: i64 = 0;
    let sig = &signal.buffer[signal.pos as usize..];

    for i in 0..FILTER_TAPS {
        e += sig[i] as i64 * coeffs[i] as i64;
    }

    rshift64_clip24(e, shift)
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos + FILTER_TAPS as u8) as usize] = sample;
    signal.pos = (signal.pos + 1) & (FILTER_TAPS as u8 - 1);
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [AptxFilterSignal; NB_FILTERS],
    coeffs: &[[i32; FILTER_TAPS]; NB_FILTERS],
    shift: u32,
    low_subband_input: i32,
    high_subband_input: i32,
    samples: &mut [Box<[i32; NB_FILTERS]>; 1],
) {
    let mut subbands = [0; NB_FILTERS];

    subbands[0] = low_subband_input + high_subband_input;
    subbands[1] = low_subband_input - high_subband_input;

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], subbands[1 - i]);
        samples[0][i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
}

fn aptx_qmf_tree_synthesis(
    qmf: &mut AptxQMFAnalysis,
    subband_samples: &[i32; NB_SUBBANDS],
    samples: &mut [Box<[i32; 2]>; 2],
) {
    for i in 0..2 {
        aptx_qmf_polyphase_synthesis(
            &mut qmf.inner_filter_signal[i],
            &APTX_QMF_INNER_COEFFS,
            22,
            subband_samples[2 * i],
            subband_samples[2 * i + 1],
            &mut [samples[i].clone()],
        );
    }

    for i in 0..2 {
        aptx_qmf_polyphase_synthesis(
            &mut qmf.outer_filter_signal,
            &APTX_QMF_OUTER_COEFFS,
            21,
            samples[i][0],
            samples[i][1],
            &mut [samples[i].clone()],
        );
    }
}

fn aptx_decode_channel(channel: &mut AptxChannel, samples: &mut [Box<[i32; 2]>; 2]) {
    let mut subband_samples = [0; NB_SUBBANDS];

    for subband in 0..NB_SUBBANDS {
        subband_samples[subband] = channel.prediction[subband].previous_reconstructed_sample;
    }

    aptx_qmf_tree_synthesis(&mut channel.qmf, &subband_samples, samples);
}


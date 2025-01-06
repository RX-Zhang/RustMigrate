

use libc::int32_t;
use std::mem;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [int32_t; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
struct AptxPrediction {
    prev_sign: [int32_t; 2],
    s_weight: [int32_t; 2],
    d_weight: [int32_t; 24],
    pos: int32_t,
    reconstructed_differences: [int32_t; 48],
    previous_reconstructed_sample: int32_t,
    predicted_difference: int32_t,
    predicted_sample: int32_t,
}

#[repr(C)]
struct AptxInvertQuantize {
    quantization_factor: int32_t,
    factor_select: int32_t,
    reconstructed_difference: int32_t,
}

#[repr(C)]
struct AptxQuantize {
    quantized_sample: int32_t,
    quantized_sample_parity_change: int32_t,
    error: int32_t,
}

#[repr(C)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
struct AptxChannel {
    codeword_history: int32_t,
    dither_parity: int32_t,
    dither: [int32_t; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_update_codeword_history(channel: &mut AptxChannel) {
    let cw = (((channel.quantize[0].quantized_sample & 3) as i32) << 0)
        + (((channel.quantize[1].quantized_sample & 2) as i32) << 1)
        + (((channel.quantize[2].quantized_sample & 1) as i32) << 3);
    channel.codeword_history = (cw << 8) + (channel.codeword_history << 4);
}

fn aptx_generate_dither(channel: &mut AptxChannel) {
    let mut subband: usize = 0;
    let mut m: i64 = 0;
    let mut d: i32 = 0;

    aptx_update_codeword_history(channel);

    m = (5184443 * ((channel.codeword_history >> 7) as i64)) as i64;
    d = ((m * 4) + (m >> 22)) as i32;
    for subband in 0..NB_SUBBANDS {
        channel.dither[subband] = (d << (23 - 5 * subband)) as i32;
    }
    channel.dither_parity = (d >> 25) & 1;
}


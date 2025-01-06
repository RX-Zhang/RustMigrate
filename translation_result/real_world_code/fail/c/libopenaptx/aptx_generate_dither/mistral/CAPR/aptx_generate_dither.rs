
pub const NB_FILTERS: usize = 2;
pub const NB_SUBBANDS: usize = 4;
pub const FILTER_TAPS: usize = 16;

pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

pub struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

pub struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

pub struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

pub struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

pub struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

pub fn aptx_update_codeword_history(channel: &mut AptxChannel) {
    let cw = (channel.quantize[0].quantized_sample & 3) << 0
        + (channel.quantize[1].quantized_sample & 2) << 1
        + (channel.quantize[2].quantized_sample & 1) << 3;
    channel.codeword_history = (cw << 8) + (channel.codeword_history << 4);
}

pub fn aptx_generate_dither(channel: &mut AptxChannel) {
    let mut subband: usize;
    let m: i64 = 5184443 * ((channel.codeword_history >> 7) as i64);
    let d: i32 = ((m * 4) + (m >> 22)) as i32;
    aptx_update_codeword_history(channel);
    for subband in 0..NB_SUBBANDS {
        channel.dither[subband] = (d << (23 - 5 * subband)) as i32;
    }
    channel.dither_parity = (d >> 25) & 1;
}

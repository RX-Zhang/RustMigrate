

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

type Int32 = i32;
type Uint8 = u8;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [Int32; 2 * FILTER_TAPS],
    pos: Uint8,
}

#[repr(C)]
struct AptxPrediction {
    prev_sign: [Int32; 2],
    s_weight: [Int32; 2],
    d_weight: [Int32; 24],
    pos: Int32,
    reconstructed_differences: [Int32; 48],
    previous_reconstructed_sample: Int32,
    predicted_difference: Int32,
    predicted_sample: Int32,
}

#[repr(C)]
struct AptxInvertQuantize {
    quantization_factor: Int32,
    factor_select: Int32,
    reconstructed_difference: Int32,
}

#[repr(C)]
struct AptxQuantize {
    quantized_sample: Int32,
    quantized_sample_parity_change: Int32,
    error: Int32,
}

#[repr(C)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
struct AptxChannel {
    codeword_history: Int32,
    dither_parity: Int32,
    dither: [Int32; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_update_codeword_history(channel: &mut AptxChannel) {
    let cw = ((channel.quantize[0].quantized_sample & 3) << 0)
        + ((channel.quantize[1].quantized_sample & 2) << 1)
        + ((channel.quantize[2].quantized_sample & 1) << 3);
    channel.codeword_history = (cw << 8) + (channel.codeword_history >> 4);
}

fn aptx_generate_dither(channel: &mut AptxChannel) {
    let m: i64 = (5184443 * (channel.codeword_history as i64 >> 7)) as i64;
    let d = ((m * 4) + (m >> 22)) as i32;
    for subband in 0..NB_SUBBANDS {
        channel.dither[subband] = (d << (23 - 5 * subband)) as i32;
    }
    channel.dither_parity = (d >> 25) & 1;
}

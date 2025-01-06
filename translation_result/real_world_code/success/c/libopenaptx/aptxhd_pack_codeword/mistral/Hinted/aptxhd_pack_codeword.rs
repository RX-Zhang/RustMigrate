
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Debug)]
struct FilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Debug)]
struct Prediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Debug)]
struct InvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Debug)]
struct Quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Debug)]
struct QmfAnalysis {
    outer_filter_signal: [FilterSignal; NB_FILTERS],
    inner_filter_signal: [[FilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Debug)]
struct Channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: QmfAnalysis,
    quantize: [Quantize; NB_SUBBANDS],
    invert_quantize: [InvertQuantize; NB_SUBBANDS],
    prediction: [Prediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &Channel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn aptxhd_pack_codeword(channel: &Channel) -> u32 {
    let parity = aptx_quantized_parity(channel);
    ((((channel.quantize[3].quantized_sample & 0x01E) | parity) as u32) << 19)
        | (((channel.quantize[2].quantized_sample & 0x00F) as u32) << 15)
        | (((channel.quantize[1].quantized_sample & 0x03F) as u32) << 9)
        | (((channel.quantize[0].quantized_sample & 0x1FF) as u32) << 0)
}

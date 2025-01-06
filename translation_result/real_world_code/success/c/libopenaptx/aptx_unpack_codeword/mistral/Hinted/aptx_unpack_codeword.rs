
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[derive(Debug)]
struct AptxPrediction {
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
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Debug)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Debug)]
struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Debug)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as usize - bits;
    let v = (val as u32) << shift;
    (v as i32) >> shift
}

fn aptx_unpack_codeword(channel: &mut AptxChannel, codeword: u16) {
    channel.quantize[0].quantized_sample = sign_extend(codeword.wrapping_shr(0) as i32, 7);
    channel.quantize[1].quantized_sample = sign_extend(codeword.wrapping_shr(7) as i32, 4);
    channel.quantize[2].quantized_sample = sign_extend(codeword.wrapping_shr(11) as i32, 2);
    channel.quantize[3].quantized_sample = sign_extend(codeword.wrapping_shr(13) as i32, 3);
    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !1)
        | aptx_quantized_parity(channel);
}

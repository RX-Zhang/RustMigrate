
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Debug)]
struct AptxFiltSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct AptxInvQuantize {
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
struct AptxQMFAnalysis {
    outer_filter_signal: Box<[AptxFiltSignal; NB_FILTERS]>,
    inner_filter_signal: Box<[[AptxFiltSignal; NB_FILTERS]; NB_FILTERS]>,
}

#[derive(Debug)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: Box<[i32; NB_SUBBANDS]>,
    qmf: AptxQMFAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;

    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }

    parity & 1
}

fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() - bits) as u32;
    let shifted = (val as u32).wrapping_shl(shift);
    ((shifted as i32) >> shift) as i32
}

fn aptxhd_unpack_codeword(channel: &mut AptxChannel, codeword: u32) {
    channel.quantize[0].quantized_sample = sign_extend((codeword >> 0) as i32, 9);
    channel.quantize[1].quantized_sample = sign_extend((codeword >> 9) as i32, 6);
    channel.quantize[2].quantized_sample = sign_extend((codeword >> 15) as i32, 4);
    channel.quantize[3].quantized_sample = sign_extend((codeword >> 19) as i32, 5);

    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !1)
        | aptx_quantized_parity(channel);
}

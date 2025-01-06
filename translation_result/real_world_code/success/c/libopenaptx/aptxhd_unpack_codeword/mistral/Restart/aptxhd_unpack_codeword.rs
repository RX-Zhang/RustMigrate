
pub const NB_FILTERS: usize = 2;
pub const NB_SUBBANDS: usize = 4;
pub const FILTER_TAPS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

pub fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

pub fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as usize - bits;
    let v = (val as u32) << shift;
    (v as i32) >> shift
}

pub fn aptxhd_unpack_codeword(channel: &mut AptxChannel, codeword: u32) {
    channel.quantize[0].quantized_sample = sign_extend((codeword >> 0) as i32, 9);
    channel.quantize[1].quantized_sample = sign_extend((codeword >> 9) as i32, 6);
    channel.quantize[2].quantized_sample = sign_extend((codeword >> 15) as i32, 4);
    channel.quantize[3].quantized_sample = sign_extend((codeword >> 19) as i32, 5);
    channel.quantize[3].quantized_sample =
        (channel.quantize[3].quantized_sample & !1) | aptx_quantized_parity(channel);
}

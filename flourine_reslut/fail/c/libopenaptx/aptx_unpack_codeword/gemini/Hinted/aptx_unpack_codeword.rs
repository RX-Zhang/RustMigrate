
use std::ops::{Add, Shr, Shl};

#[derive(Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone, Copy)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone, Copy)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 4],
    inner_filter_signal: [[AptxFilterSignal; 4]; 4],
}

#[derive(Clone, Copy)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 4],
    invert_quantize: [AptxInvertQuantize; 4],
    prediction: [AptxPrediction; 4],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..4 {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let mut v = val as u32;
    v <<= shift;
    (v as i32) >> shift
}

fn aptx_unpack_codeword(channel: &mut AptxChannel, codeword: u16) {
    channel.quantize[0].quantized_sample = sign_extend(codeword as i32 >> 0, 7);
    channel.quantize[1].quantized_sample = sign_extend((codeword as i32 >> 7) as i32, 4);
    channel.quantize[2].quantized_sample = sign_extend((codeword as i32 >> 11) as i32, 2) as i32;
    channel.quantize[3].quantized_sample = sign_extend(codeword as i32 >> 13, 3) as i32;
    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !1)
        | aptx_quantized_parity(channel);
}

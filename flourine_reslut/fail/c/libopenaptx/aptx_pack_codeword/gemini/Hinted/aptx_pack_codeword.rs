
use std::ops::{Add, BitXor};

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 12],
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
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 4],
    inner_filter_signal: [[AptxFilterSignal; 4]; 4],
}

#[derive(Debug)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 4],
    invert_quantize: [AptxInvertQuantize; 4],
    prediction: [AptxPrediction; 4],
}

const NB_SUBBANDS: usize = 4;

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn aptx_pack_codeword(channel: &AptxChannel) -> u16 {
    let parity = aptx_quantized_parity(channel);
    let mut codeword: u16 = 0;
    codeword.wrapping_add((((channel.quantize[3].quantized_sample & 0x06) | parity) as u16) << 13);
    codeword.wrapping_add((((channel.quantize[2].quantized_sample & 0x03) as u16) << 11));
    codeword.wrapping_add((((channel.quantize[1].quantized_sample & 0x0F) as u16) << 7));
    codeword.wrapping_add((((channel.quantize[0].quantized_sample & 0x7F) as u16) << 0));
    codeword
}

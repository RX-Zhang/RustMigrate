

use std::mem;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
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

#[repr(C)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
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
    const SHIFT: i32 = 8 * std::mem::size_of::<i32>() as i32 - 8;
    let mut v = (val as u32) << SHIFT;
    i32::from(unsafe { mem::transmute::<u32, i32>(v >> SHIFT) })
}

fn aptxhd_unpack_codeword(channel: &mut AptxChannel, codeword: u32) {
    channel.quantize[0].quantized_sample = sign_extend((codeword >> 0) as i32, 9).wrapping_add(128) - 128;
    channel.quantize[1].quantized_sample = sign_extend((codeword >> 9) as i32, 6).wrapping_add(16) - 16;
    channel.quantize[2].quantized_sample = sign_extend((codeword >> 15) as i32, 4).wrapping_add(8) - 8;
    channel.quantize[3].quantized_sample = (sign_extend((codeword >> 19) as i32, 5) & !1)
        .wrapping_add(16)
        .wrapping_sub(16)
        .wrapping_add(aptx_quantized_parity(channel));
}


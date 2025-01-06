
use std::ptr;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
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
struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; 4],
    inner_filter_signal: [[AptxFilterSignal; 4]; 4],
}

#[repr(C)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],

    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; 4],
    invert_quantize: [AptxInvertQuantize; 4],
    prediction: [AptxPrediction; 4],
}

fn aptx_update_codeword_history(channel: &mut AptxChannel) {
    let cw = ((channel.quantize[0].quantized_sample & 3) << 0)
        + ((channel.quantize[1].quantized_sample & 2) << 1)
        + ((channel.quantize[2].quantized_sample & 1) << 3);
    channel.codeword_history = (cw << 8) + ((channel.codeword_history as u32) << 4) as i32;
}

fn aptx_generate_dither(channel: &mut AptxChannel) {
    let mut subband = 0;
    let mut m: i64;
    let mut d: i32;

    aptx_update_codeword_history(channel);

    m = 5184443 * (channel.codeword_history >> 7) as i64;
    d = ((m * 4) + (m >> 22)) as i32;
    while subband < 4 {
        channel.dither[subband] = ((d as u32) << (23 - 5 * subband)) as i32;
        subband += 1;
    }
    channel.dither_parity = (d >> 25) & 1;
}

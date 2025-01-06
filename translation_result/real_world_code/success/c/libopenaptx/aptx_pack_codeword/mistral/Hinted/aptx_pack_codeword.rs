
pub const NB_FILTERS: usize = 2;
pub const NB_SUBBANDS: usize = 4;
pub const FILTER_TAPS: usize = 16;

#[derive(Debug)]
pub struct AptxFilterSignal {
    pub buffer: [i32; 2 * FILTER_TAPS],
    pub pos: u8,
}

#[derive(Debug)]
pub struct AptxPrediction {
    pub prev_sign: [i32; 2],
    pub s_weight: [i32; 2],
    pub d_weight: [i32; 24],
    pub pos: i32,
    pub reconstructed_differences: [i32; 48],
    pub previous_reconstructed_sample: i32,
    pub predicted_difference: i32,
    pub predicted_sample: i32,
}

#[derive(Debug)]
pub struct AptxInvertQuantize {
    pub quantization_factor: i32,
    pub factor_select: i32,
    pub reconstructed_difference: i32,
}

#[derive(Debug)]
pub struct AptxQuantize {
    pub quantized_sample: i32,
    pub quantized_sample_parity_change: i32,
    pub error: i32,
}

#[derive(Debug)]
pub struct AptxQmfAnalysis {
    pub outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    pub inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Debug)]
pub struct AptxChannel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; NB_SUBBANDS],
    pub qmf: AptxQmfAnalysis,
    pub quantize: [AptxQuantize; NB_SUBBANDS],
    pub invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    pub prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn aptx_pack_codeword(channel: &AptxChannel) -> u16 {
    let parity = aptx_quantized_parity(channel);
    ((((channel.quantize[3].quantized_sample & 0x06) | parity) as u16) << 13)
        | (((channel.quantize[2].quantized_sample & 0x03) as u16) << 11)
        | (((channel.quantize[1].quantized_sample & 0x0F) as u16) << 7)
        | (((channel.quantize[0].quantized_sample & 0x7F) as u16) << 0)
}

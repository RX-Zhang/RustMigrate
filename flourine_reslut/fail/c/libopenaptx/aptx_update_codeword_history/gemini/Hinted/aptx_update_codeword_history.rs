
#[repr(C)]
pub struct AptxFilterSignal {
    pub buffer: [i32; 2 * 4],
    pub pos: u8,
}

#[repr(C)]
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

#[repr(C)]
pub struct AptxInvertQuantize {
    pub quantization_factor: i32,
    pub factor_select: i32,
    pub reconstructed_difference: i32,
}

#[repr(C)]
pub struct AptxQuantize {
    pub quantized_sample: i32,
    pub quantized_sample_parity_change: i32,
    pub error: i32,
}

#[repr(C)]
pub struct AptxQmfAnalysis {
    pub outer_filter_signal: [AptxFilterSignal; 4],
    pub inner_filter_signal: [[AptxFilterSignal; 4]; 4],
}

#[repr(C)]
pub struct AptxChannel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; 4],
    pub qmf: AptxQmfAnalysis,
    pub quantize: [AptxQuantize; 4],
    pub invert_quantize: [AptxInvertQuantize; 4],
    pub prediction: [AptxPrediction; 4],
}

#[inline]
pub fn aptx_update_codeword_history(channel: &mut AptxChannel) {
    let cw = ((channel.quantize[0].quantized_sample & 3) << 0)
        + ((channel.quantize[1].quantized_sample & 2) << 1)
        + ((channel.quantize[2].quantized_sample & 1) << 3);
    channel.codeword_history = (cw << 8) + ((channel.codeword_history as u32) << 4) as i32;
}

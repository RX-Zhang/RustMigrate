
use std::ops::{Add, Mul};

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
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
    outer_filter_signal: [AptxFilterSignal; 16],
    inner_filter_signal: [[AptxFilterSignal; 16]; 16],
}

#[derive(Debug)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 16],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 16],
    invert_quantize: [AptxInvertQuantize; 16],
    prediction: [AptxPrediction; 16],
}

impl AptxChannel {
    fn update_codeword_history(&mut self) {
        let cw = ((self.quantize[0].quantized_sample & 3) << 0)
            + ((self.quantize[1].quantized_sample & 2) << 1)
            + ((self.quantize[2].quantized_sample & 1) << 3);
        self.codeword_history = (cw << 8) + ((self.codeword_history as i32) << 4);
    }
}

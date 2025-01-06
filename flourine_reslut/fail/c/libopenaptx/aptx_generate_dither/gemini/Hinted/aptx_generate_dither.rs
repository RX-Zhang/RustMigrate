

use std::ops::{Add, AddAssign};

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 16],
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
    outer_filter_signal: [AptxFilterSignal; 2],
    inner_filter_signal: [[AptxFilterSignal; 2]; 2],
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

impl AptxChannel {
    fn update_codeword_history(&mut self) {
        let cw = ((self.quantize[0].quantized_sample & 3) << 0)
            + ((self.quantize[1].quantized_sample & 2) << 1)
            + ((self.quantize[2].quantized_sample & 1) << 3);
        self.codeword_history = (cw << 8) + ((self.codeword_history as i32) << 4);
    }

    fn generate_dither(&mut self) {
        let mut m: i64 = (5184443 * (self.codeword_history >> 7)).into();
        let d = ((m * 4) + (m >> 22)) as i32;
        for subband in 0..4 {
            self.dither[subband] = (d as i32) << (23 - 5 * subband);
        }
        self.dither_parity = (d >> 25) & 1;
    }
}

impl Add for AptxFilterSignal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut buffer = [0; 16];
        for i in 0..16 {
            buffer[i] = self.buffer[i].wrapping_add(rhs.buffer[i]);
        }
        AptxFilterSignal {
            buffer,
            pos: self.pos.wrapping_add(rhs.pos),
        }
    }
}

impl AddAssign for AptxFilterSignal {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self.buffer[i] = self.buffer[i].wrapping_add(rhs.buffer[i]);
        }
        self.pos = self.pos.wrapping_add(rhs.pos);
    }
}


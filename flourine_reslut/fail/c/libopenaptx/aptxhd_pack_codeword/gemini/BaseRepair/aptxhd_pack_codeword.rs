
use std::ops::{Add, AddAssign};

#[derive(Clone)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

#[derive(Clone)]
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

#[derive(Clone)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Clone)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Clone)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 4],
    inner_filter_signal: [[AptxFilterSignal; 4]; 4],
}

#[derive(Clone)]
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
    fn aptx_quantized_parity(&self) -> i32 {
        let mut parity = self.dither_parity;
        for subband in 0..4 {
            parity ^= self.quantize[subband].quantized_sample;
        }
        parity & 1
    }

    fn aptxhd_pack_codeword(&self) -> u32 {
        let parity = self.aptx_quantized_parity();
        (((((self.quantize[3].quantized_sample & 0x01E) | parity) << 19)
            | (((self.quantize[2].quantized_sample & 0x00F) << 15)
            | (((self.quantize[1].quantized_sample & 0x03F) << 9)
            | (((self.quantize[0].quantized_sample & 0x1FF) << 0)))))) as u32
    }
}

impl Add for AptxFilterSignal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new_buffer = [0; 2 * 16];
        for i in 0..2 * 16 {
            new_buffer[i] = self.buffer[i].wrapping_add(other.buffer[i]);
        }
        AptxFilterSignal {
            buffer: new_buffer,
            pos: self.pos.wrapping_add(other.pos),
        }
    }
}

impl AddAssign for AptxFilterSignal {
    fn add_assign(&mut self, other: Self) {
        for i in 0..2 * 16 {
            self.buffer[i] = self.buffer[i].wrapping_add(other.buffer[i]);
        }
        self.pos = self.pos.wrapping_add(other.pos);
    }
}

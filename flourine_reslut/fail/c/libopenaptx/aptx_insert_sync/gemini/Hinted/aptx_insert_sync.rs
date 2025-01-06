
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy)]
enum Channels {
    Left,
    Right,
}

struct AptxFilterSignal {
    buffer: [i32; 2 * 32],
    pos: u8,
}

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

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 5],
    inner_filter_signal: [[AptxFilterSignal; 5]; 5],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 5],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 5],
    invert_quantize: [AptxInvertQuantize; 5],
    prediction: [AptxPrediction; 5],
}

impl Clone for AptxQuantize {
    fn clone(&self) -> Self {
        Self {
            quantized_sample: self.quantized_sample,
            quantized_sample_parity_change: self.quantized_sample_parity_change,
            error: self.error,
        }
    }
}

impl Add for AptxQuantize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            quantized_sample: self.quantized_sample + rhs.quantized_sample,
            quantized_sample_parity_change: self.quantized_sample_parity_change
                + rhs.quantized_sample_parity_change,
            error: self.error + rhs.error,
        }
    }
}

impl AddAssign for AptxQuantize {
    fn add_assign(&mut self, rhs: Self) {
        self.quantized_sample += rhs.quantized_sample;
        self.quantized_sample_parity_change += rhs.quantized_sample_parity_change;
        self.error += rhs.error;
    }
}

const NB_SUBBANDS: usize = 5;
const NB_CHANNELS: usize = 2;
const NB_FILTERS: usize = 5;
const FILTER_TAPS: usize = 32;

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; NB_CHANNELS], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize]);
    let eighth = *sync_idx == 7;
    *sync_idx = (*sync_idx + 1) & 7;
    parity ^ eighth as i32
}

fn aptx_insert_sync(channels: &mut [AptxChannel; NB_CHANNELS], sync_idx: &mut u8) {
    let mut min = channels[NB_CHANNELS - 1].quantize[0].clone();
    for c in channels.iter_mut().rev() {
        for i in 0..NB_SUBBANDS {
            if c.quantize[i].error < min.error {
                min = c.quantize[i].clone();
            }
        }
    }
    if aptx_check_parity(channels, sync_idx) != 0 {
        min.quantized_sample = min.quantized_sample_parity_change;
    }
}

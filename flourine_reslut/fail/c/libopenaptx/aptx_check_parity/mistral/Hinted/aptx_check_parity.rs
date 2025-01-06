

use std::convert::TryInto;
use std::mem;
use std::usize;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Channel {
    Left = 0,
    Right,
    NbChannels,
}

#[derive(Default)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: i32,
}

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [[i32; 12]; 4],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Default)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Default)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Default)]
struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[derive(Default)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

impl AptxChannel {
    fn aptx_quantized_parity(&self) -> i32 {
        let mut parity = self.dither_parity;

        for subband in 0..NB_SUBBANDS {
            parity ^= self.quantize[subband].quantized_sample;
        }

        parity & 1
    }

    fn aptx_check_parity(channels: &[AptxChannel; 2], sync_idx: &mut u8) -> i32 {
        let parity = channels
            .get(Channel::Left as usize)
            .unwrap()
            .aptx_quantized_parity()
            ^ channels
                .get(Channel::Right as usize)
                .unwrap()
                .aptx_quantized_parity();
        let eighth = (*sync_idx == 7) as i32;

        *sync_idx = (*sync_idx + 1) % 8;
        parity ^ eighth
    }
}

fn main() {}




use std::mem;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

enum Channels {
    Left,
    Right,
    NbChannels,
}

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

struct AptxPrediction {
    prev_sign: Box<[i32; 2]>,
    s_weight: Box<[i32; 2]>,
    d_weight: Box<[i32; 24]>,
    pos: i32,
    reconstructed_differences: Box<[i32; 48]>,
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
    outer_filter_signal: Box<[AptxFilterSignal; NB_FILTERS]>,
    inner_filter_signal: Box<[Box<[AptxFilterSignal; NB_FILTERS]>; NB_FILTERS]>,
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: Box<[i32; NB_SUBBANDS]>,
    qmf: AptxQMFAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvertQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    let mut subband = 0;

    while subband < NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
        subband += 1;
    }

    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; 2], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize]);
    let eighth = *sync_idx == 7;

    *sync_idx = (*sync_idx).wrapping_add(1) & 7;
    parity ^ (eighth as i32)
}

fn aptx_insert_sync(channels: &mut [AptxChannel; 2], sync_idx: &mut u8) {
    let map = [1, 2, 0, 3];
    let mut min_error = i32::MAX;
    let mut min_idx = 0;

    for (c_idx, c) in channels.iter_mut().enumerate() {
        for (i, &m) in map.iter().enumerate() {
            if c.quantize[m].error < min_error {
                min_error = c.quantize[m].error;
                min_idx = c_idx * NB_SUBBANDS + i;
            }
        }
    }

    if aptx_check_parity(channels, sync_idx) != 0 {
        let (c_idx, i) = (min_idx / NB_SUBBANDS, min_idx % NB_SUBBANDS);
        channels[c_idx].quantize[i].quantized_sample = channels[c_idx].quantize[i].quantized_sample_parity_change;
    }
}


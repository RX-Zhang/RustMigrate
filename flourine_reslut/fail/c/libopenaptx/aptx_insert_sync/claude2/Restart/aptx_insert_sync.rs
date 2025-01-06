

use std::convert::TryInto;

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

struct AptxQmfAnalysis {
    outer_filter_signal: Box<[AptxFilterSignal; NB_FILTERS]>,
    inner_filter_signal: Box<[[AptxFilterSignal; NB_FILTERS]; NB_FILTERS]>,
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: Box<[i32; NB_SUBBANDS]>,

    qmf: AptxQmfAnalysis,
    quantize: Box<[AptxQuantize; NB_SUBBANDS]>,
    invert_quantize: Box<[AptxInvertQuantize; NB_SUBBANDS]>,
    prediction: Box<[AptxPrediction; NB_SUBBANDS]>,
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for quantize in channel.quantize.iter() {
        parity ^= quantize.quantized_sample & 1;
    }
    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; Channels::NbChannels as usize], sync_idx: &mut u8) -> bool {
    let parity = (aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize])) != 0;
    let eighth = *sync_idx == 7;

    *sync_idx = sync_idx.wrapping_add(1) & 7;
    parity ^ eighth
}

fn aptx_insert_sync(channels: &mut [AptxChannel; Channels::NbChannels as usize], sync_idx: &mut u8) {
    let map: Box<[usize; 4]> = Box::new([1, 2, 0, 3]);
    let (mut min, mut min_idx) = (std::i32::MAX, 0);
    for (i, channel) in channels.iter_mut().enumerate().rev() {
        for &j in map.iter() {
            if channel.quantize[j].error < min {
                min = channel.quantize[j].error;
                min_idx = i;
            }
        }
    }

    if aptx_check_parity(channels, sync_idx) {
        channels[min_idx].quantize[map[0]].quantized_sample = channels[min_idx].quantize[map[0]].quantized_sample_parity_change;
    }
}



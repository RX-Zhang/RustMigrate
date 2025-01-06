
use std::ops::BitXor;
use std::u8;

const NB_FILTERS: usize =2;
const NB_SUBBANDS: usize = 4;
const FTER_TAPS: usize = 16;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Channel {
    Left  = 0,
    Right = 1,
    NbChannels,
}

struct AptxFilterSignal {
    buffer: [i32; 2 * FTER_TAPS],
    pos: u8,
}

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: u8,
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
    filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity = parity.bitxor(channel.quantize[subband].quantized_sample);
    }
    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; 2], sync_idx: &mut u8) -> bool {
    let parity = (aptx_quantized_parity(&channels[Channel::Left as usize])
        ^ aptx_quantized_parity(&channels[Channel::Right as usize]))
        as u8;
    let eighth = (*sync_idx == 7) as u8;

    *sync_idx = (*sync_idx + 1) & 7;
    parity == eighth
}

fn aptx_insert_sync(channels: &mut [AptxChannel; 2], sync_idx: &mut u8) {
    let mut min_error = i32::MAX;
    let mut min_quantize_index = (0, 0);
    for c_index in (0..2).rev() {
        let c = &mut channels[c_index];
        for subband in 0..NB_SUBBANDS {
            let quantize = &mut c.quantize[subband];
            if quantize.error < min_error {
                min_error = quantize.error;
                min_quantize_index = (c_index, subband);
            }
        }
    }

    if aptx_check_parity(channels, sync_idx) {
        let (c_index, subband) = min_quantize_index;
        let min_quantize = &mut channels[c_index].quantize[subband];
        min_quantize.quantized_sample =
            min_quantize.quantized_sample_parity_change;
    }
}

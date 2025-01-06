

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[derive(Clone, Copy)]
enum Channels {
    Left,
    Right,
}

const NB_CHANNELS: usize = 2;

struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
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

struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }

    parity & 1
}

fn aptx_check_parity(channels: &[AptxChannel; NB_CHANNELS], sync_idx: &mut u8) -> bool {
    let parity = aptx_quantized_parity(&channels[Channels::Left as usize])
        ^ aptx_quantized_parity(&channels[Channels::Right as usize]);
    let eighth = (*sync_idx == 7) as i32;

    *sync_idx = (*sync_idx).wrapping_add(1) & 7;
    (parity ^ eighth) != 0
}

fn aptx_insert_sync(channels: &mut [AptxChannel; NB_CHANNELS], sync_idx: &mut u8) {
    let map: [usize; 4] = [1, 2, 0, 3];
    
    if aptx_check_parity(channels, sync_idx) {
        let mut min_error = i32::MAX;
        let mut min_channel = 0;
        let mut min_subband = 0;

        for (c, channel) in channels.iter().enumerate().rev() {
            for i in 0..NB_SUBBANDS {
                let error = channel.quantize[map[i]].error;
                if error < min_error {
                    min_error = error;
                    min_channel = c;
                    min_subband = map[i];
                }
            }
        }

        channels[min_channel].quantize[min_subband].quantized_sample = 
            channels[min_channel].quantize[min_subband].quantized_sample_parity_change;
    }
}

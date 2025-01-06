

use std::convert::TryInto;

const LATENCY_SAMPLES: usize = 90;
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
    inner_filter_signal: Box<[Box<[AptxFilterSignal; NB_FILTERS]>; NB_FILTERS]>,
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

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: Box<[AptxChannel; Channels::NbChannels as usize]>,
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: Box<[u8; 6]>,
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;

    for i in 0..std::mem::size_of::<AptxContext>() {
        unsafe {
            *((ctx as *mut AptxContext)
                .wrapping_add(i)
                as *mut u8) = 0;
        }
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in 0..Channels::NbChannels as usize {
        let prediction = &mut ctx.channels[chan].prediction;
        for subband in 0..NB_SUBBANDS {
            *prediction[subband].prev_sign.get_mut(1).unwrap() = 1;
        }
    }
}

fn aptx_decode_sync_finish(ctx: &mut AptxContext) -> usize {
    let dropped = ctx.decode_sync_buffer_len.try_into().unwrap();
    aptx_reset(ctx);
    dropped
}


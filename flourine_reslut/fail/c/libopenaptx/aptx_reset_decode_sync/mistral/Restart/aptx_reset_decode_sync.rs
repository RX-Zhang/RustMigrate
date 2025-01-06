

use std::mem;

const LATENCY_SAMPLES: u32 = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[repr(C)]
pub enum Channels {
    Left,
    Right,
    NbChannels,
}

#[repr(C)]
pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
pub struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[repr(C)]
pub struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
pub struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
pub struct AptxQMFAnalysis {
    outer_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
    inner_filter_signal: [[[AptxFilterSignal; NB_FILTERS]; NB_FILTERS]; NB_SUBBANDS],
}

#[repr(C)]
pub struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

#[repr(C)]
pub struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; 2],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

pub fn aptx_reset(ctx: &mut AptxContext) {
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    ctx.hd = ctx.hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in ctx.channels.iter_mut() {
        for subband in 0..NB_SUBBANDS {
            let prediction = &mut chan.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

pub fn aptx_reset_decode_sync(ctx: &mut AptxContext) {
    let decode_dropped = ctx.decode_dropped;
    let decode_sync_packets = ctx.decode_sync_packets;
    let decode_sync_buffer_len = ctx.decode_sync_buffer_len;
    let mut decode_sync_buffer = [0u8; 6];
    let i = 0;

    for i in 0..6 {
        decode_sync_buffer[i] = ctx.decode_sync_buffer[i];
    }

    aptx_reset(ctx);

    for i in 0..6 {
        ctx.decode_sync_buffer[i] = decode_sync_buffer[i];
    }

    ctx.decode_sync_buffer_len = decode_sync_buffer_len;
    ctx.decode_sync_packets = decode_sync_packets;
    ctx.decode_dropped = decode_dropped;
}


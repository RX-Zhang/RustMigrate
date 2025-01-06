

use std::alloc::{GlobalAlloc, Layout, System};

#[global_allocator]
static ALLOC: System = System;

const LATENCY_SAMPLES: u8 = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;

enum Channels {
    Left,
    Right,
    NbChannels,
}

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

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; Channels::NbChannels as usize],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    for i in 0..std::mem::size_of::<AptxContext>() {
        unsafe {
            *((ctx as *mut AptxContext)
                .wrapping_add(i) as *mut u8) = 0;
        }
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = (LATENCY_SAMPLES + 3) as u8 / 4;
    ctx.encode_remaining = (LATENCY_SAMPLES + 3) as u8 / 4;

    for chan in 0..Channels::NbChannels as usize {
        let channel = &mut ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            let prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

fn aptx_init(hd: i32) -> Box<AptxContext> {
    let mut ctx: Box<AptxContext> = Box::new(AptxContext{
        decode_sync_packets: 0,
        decode_dropped: 0,
        channels: unsafe { std::mem::zeroed() },
        hd: 0,
        sync_idx: 0,
        encode_remaining: 0,
        decode_skip_leading: 0,
        decode_sync_buffer_len: 0,
        decode_sync_buffer: [0; 6],
    });

    ctx.hd = hd as u8;

    aptx_reset(&mut ctx);

    ctx
}


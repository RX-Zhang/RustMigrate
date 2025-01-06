

use std::boxed::Box;

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
    outer_filter_signal: [Box<AptxFilterSignal>; NB_FILTERS],
    inner_filter_signal: [[Box<AptxFilterSignal>; NB_FILTERS]; NB_FILTERS],  
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32, 
    dither: [i32; NB_SUBBANDS],
    
    qmf: Box<AptxQmfAnalysis>,
    quantize: [Box<AptxQuantize>; NB_SUBBANDS],
    invert_quantize: [Box<AptxInvertQuantize>; NB_SUBBANDS],
    prediction: [Box<AptxPrediction>; NB_SUBBANDS],
}

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,  
    channels: [Box<AptxChannel>; Channels::NbChannels as usize],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],  
}

fn aptx_reset(ctx: &mut AptxContext) {
    ctx.hd = ctx.hd;
    
    for chan in &mut ctx.channels {
        for subband in &mut chan.prediction {
            subband.prev_sign = [1, 1];
        }
    }

    ctx.decode_sync_packets = 0;
    ctx.decode_dropped = 0;
    ctx.sync_idx = 0;
    ctx.encode_remaining = LATENCY_SAMPLES.wrapping_add(3) / 4;
    ctx.decode_skip_leading = LATENCY_SAMPLES.wrapping_add(3) / 4;
    ctx.decode_sync_buffer_len = 0;
    ctx.decode_sync_buffer = [0; 6];
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


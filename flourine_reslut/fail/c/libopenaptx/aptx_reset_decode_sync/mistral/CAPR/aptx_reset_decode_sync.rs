

use std::mem;

const LATENCY_SAMPLES: usize = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

struct AptxPrediction {
    prev_sign: [i16; 2],
}

struct AptxChannel {
    prediction: [AptxPrediction; NB_SUBBANDS],
}

struct AptxContext {
    hd: i32,
    decode_skip_leading: u8,
    encode_remaining: u8,
    channels: [AptxChannel; 2],
    decode_sync_buffer: [u8; 6],
    decode_sync_buffer_len: usize,
    decode_sync_packets: i32,
    decode_dropped: i32,
}

impl AptxContext {
    fn zeroed() -> AptxContext {
        unsafe { mem::zeroed() }
    }
}

pub fn aptx_reset(ctx: &mut AptxContext) {
    *ctx = AptxContext::zeroed();

    ctx.hd = ctx.hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3).wrapping_add(3)) as u8 / 4;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3).wrapping_add(3)) as u8 / 4;

    let mut channel = &mut ctx.channels[..];
    for channel in channel.iter_mut() {
        let mut prediction = &mut channel.prediction[..];
        for prediction in prediction.iter_mut() {
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
    let mut i: usize;

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


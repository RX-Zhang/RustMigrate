
use std::mem;

#[derive(Copy, Clone)]
struct aptx_filter_signal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

#[derive(Copy, Clone)]
struct aptx_prediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Copy, Clone)]
struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Copy, Clone)]
struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Copy, Clone)]
struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; 2],
    inner_filter_signal: [[aptx_filter_signal; 2]; 2],
}

#[derive(Copy, Clone)]
struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 12],

    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; 12],
    invert_quantize: [aptx_invert_quantize; 12],
    prediction: [aptx_prediction; 12],
}

#[derive(Copy, Clone)]
struct aptx_context {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [aptx_channel; 2],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

const LATENCY_SAMPLES: usize = 128;

fn aptx_reset(ctx: &mut aptx_context) {
    let hd = ctx.hd;
    let mut i: usize;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut aptx_channel;
    let mut prediction: &mut aptx_prediction;

    for i in 0..mem::size_of::<aptx_context>() {
        unsafe {
            *(((ctx as *mut aptx_context as *mut u8).offset(i as isize)) as *mut u8) = 0;
        }
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = (LATENCY_SAMPLES + 3) as u8 / 4;
    ctx.encode_remaining = (LATENCY_SAMPLES + 3) as u8 / 4;

    for chan in 0..2 {
        channel = &mut ctx.channels[chan];
        for subband in 0..12 {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

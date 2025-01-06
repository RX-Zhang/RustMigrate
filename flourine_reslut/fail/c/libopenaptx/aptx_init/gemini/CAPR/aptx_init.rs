
use std::boxed::Box;

#[derive(Default)]
struct AptxFilterSignal {
    buffer: [i32; 32],
    pos: u8,
}

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: Box<[i32]>,
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

#[derive(Default)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[derive(Default)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Default)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 4],
    inner_filter_signal: [[AptxFilterSignal; 4]; 4],
}

#[derive(Default)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 8],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 8],
    invert_quantize: [AptxInvertQuantize; 8],
    prediction: [AptxPrediction; 8],
}

#[derive(Default)]
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

const LATENCY_SAMPLES: usize = 12;
const NB_CHANNELS: usize = 2;

pub fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut AptxChannel;
    let mut prediction: &mut AptxPrediction;

    for i in 0..std::mem::size_of::<AptxContext>() {
        unsafe {
            *(&mut ctx.decode_sync_buffer[i] as *mut u8) = 0;
        }
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = (LATENCY_SAMPLES + 3) as u8 / 4;
    ctx.encode_remaining = (LATENCY_SAMPLES + 3) as u8 / 4;

    for chan in 0..NB_CHANNELS {
        channel = &mut ctx.channels[chan];
        for subband in 0..8 {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

pub fn aptx_init(hd: i32) -> Box<AptxContext> {
    let mut ctx: Box<AptxContext> = Box::new(AptxContext::default());

    ctx.hd = if hd != 0 { 1 } else { 0 };

    aptx_reset(&mut *ctx);
    ctx
}

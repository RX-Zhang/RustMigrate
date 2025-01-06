

use std::mem;

const LATENCY_SAMPLES: u8 = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
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

#[repr(C)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],

    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

#[repr(C)]
struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; 2],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: usize,
    decode_sync_buffer: [u8; 6],
}

extern "C" {
    fn memset(dest: *mut libc::c_void, c: libc::c_int, n: libc::size_t);
}

fn aptx_reset(ctx: &mut AptxContext) {
    unsafe {
        memset(
            ctx as *mut _ as *mut libc::c_void,
            0,
            mem::size_of::<AptxContext>(),
        );
    }

    let hd = ctx.hd;
    let chan = 0;
    let subband = 0;
    let ref mut channel = ctx.channels[chan];
    let ref mut prediction = channel.prediction[subband];

    ctx.hd = hd;
    ctx.decode_skip_leading = (LATENCY_SAMPLES.wrapping_add(3)).wrapping_div(4);
    ctx.encode_remaining = (LATENCY_SAMPLES.wrapping_add(3)).wrapping_div(4);

    for chan in 0..2 {
        let ref mut channel = ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            let ref mut prediction = channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

extern "C" fn aptx_decode_sync_finish(ctx: &mut AptxContext) -> usize {
    let dropped = ctx.decode_sync_buffer_len;
    aptx_reset(ctx);
    dropped
}


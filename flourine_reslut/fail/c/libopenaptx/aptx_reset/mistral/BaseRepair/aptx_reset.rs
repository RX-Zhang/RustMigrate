

use std::mem;

const LATENCY_SAMPLES: u32 = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;

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
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

extern "C" {
    fn aptx_reset(ctx: *mut AptxContext);
}

impl AptxContext {
    fn new() -> Self {
        AptxContext {
            decode_sync_packets: 0,
            decode_dropped: 0,
            channels: unsafe { mem::zeroed() },
            hd: 0,
            sync_idx: 0,
            encode_remaining: (LATENCY_SAMPLES + 3) as u8,
            decode_skip_leading: (LATENCY_SAMPLES + 3) as u8 / 4,
            decode_sync_buffer_len: 0,
            decode_sync_buffer: unsafe { mem::zeroed() },
        }
    }
}

#[no_mangle]
pub extern "C" fn aptx_reset_refactored(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize = 0;
    let mut chan: usize = 0;
    let mut subband: usize = 0;
    let channel: &mut AptxChannel = &mut ctx.channels[0];
    let prediction: &mut AptxPrediction = &mut channel.prediction[0];

    for i in 0..mem::size_of::<AptxContext>() {
        unsafe {
            (ctx as *mut _ as *mut u8).add(i).write_volatile(0);
        }
    }

    ctx.hd = hd;
    ctx.encode_remaining = (LATENCY_SAMPLES + 3) as u8;
    ctx.decode_skip_leading = (LATENCY_SAMPLES + 3) as u8 / 4;

    for chan in 0..2 {
        for subband in 0..4 {
            let prediction = &mut ctx.channels[chan].prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}


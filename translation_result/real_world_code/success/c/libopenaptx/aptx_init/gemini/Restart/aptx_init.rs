
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

const LATENCY_SAMPLES: usize = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Channels {
    Left = 0,
    Right = 1,
    NbChannels = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct AptxQmfAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQmfAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
    unsafe {
        ptr::write_bytes(ctx as *mut _ as *mut u8, 0, std::mem::size_of::<AptxContext>());
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in 0..Channels::NbChannels as usize {
        let channel = &mut ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            let prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

fn aptx_init(hd: bool) -> Option<Box<AptxContext>> {
    let layout = Layout::new::<AptxContext>();
    let ptr = unsafe { alloc(layout) };

    if ptr.is_null() {
        return None;
    }

    let mut ctx = unsafe { Box::from_raw(ptr as *mut AptxContext) };
    ctx.hd = if hd { 1 } else { 0 };

    aptx_reset(&mut *ctx);
    Some(ctx)
}

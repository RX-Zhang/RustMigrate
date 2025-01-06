
use std::mem;

const LATENCY_SAMPLES: usize = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const NB_CHANNELS: usize = 2;

#[derive(Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

impl Default for AptxFilterSignal {
    fn default() -> Self {
        AptxFilterSignal {
            buffer: [0; 2 * FILTER_TAPS],
            pos: 0,
        }
    }
}

#[derive(Clone, Copy)]
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

impl Default for AptxPrediction {
    fn default() -> Self {
        AptxPrediction {
            prev_sign: [1, 1],
            s_weight: [0, 0],
            d_weight: [0; 24],
            pos: 0,
            reconstructed_differences: [0; 48],
            previous_reconstructed_sample: 0,
            predicted_difference: 0,
            predicted_sample: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

impl Default for AptxInvertQuantize {
    fn default() -> Self {
        AptxInvertQuantize {
            quantization_factor: 0,
            factor_select: 0,
            reconstructed_difference: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

impl Default for AptxQuantize {
    fn default() -> Self {
        AptxQuantize {
            quantized_sample: 0,
            quantized_sample_parity_change: 0,
            error: 0,
        }
    }
}

#[derive(Clone, Copy)]
struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],
}

impl Default for AptxQMFAnalysis {
    fn default() -> Self {
        AptxQMFAnalysis {
            outer_filter_signal: [AptxFilterSignal::default(); NB_FILTERS],
            inner_filter_signal: [[AptxFilterSignal::default(); NB_FILTERS]; NB_FILTERS],
        }
    }
}

#[derive(Clone, Copy)]
struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; NB_SUBBANDS],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; NB_SUBBANDS],
    invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    prediction: [AptxPrediction; NB_SUBBANDS],
}

impl Default for AptxChannel {
    fn default() -> Self {
        AptxChannel {
            codeword_history: 0,
            dither_parity: 0,
            dither: [0; NB_SUBBANDS],
            qmf: AptxQMFAnalysis::default(),
            quantize: [AptxQuantize::default(); NB_SUBBANDS],
            invert_quantize: [AptxInvertQuantize::default(); NB_SUBBANDS],
            prediction: [AptxPrediction::default(); NB_SUBBANDS],
        }
    }
}

#[derive(Clone, Copy)]
struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; NB_CHANNELS],
    hd: u8,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

impl Default for AptxContext {
    fn default() -> Self {
        AptxContext {
            decode_sync_packets: 0,
            decode_dropped: 0,
            channels: [AptxChannel::default(); NB_CHANNELS],
            hd: 0,
            sync_idx: 0,
            encode_remaining: 0,
            decode_skip_leading: 0,
            decode_sync_buffer_len: 0,
            decode_sync_buffer: [0; 6],
        }
    }
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize = 0;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut AptxChannel;
    let mut prediction: &mut AptxPrediction;

    while i < mem::size_of::<AptxContext>() {
        unsafe {
            let ptr = ctx as *mut AptxContext as *mut u8;
            *ptr.offset(i as isize) = 0;
        }
        i += 1;
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES + 3) / 4) as u8;

    for chan in 0..NB_CHANNELS {
        channel = &mut ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

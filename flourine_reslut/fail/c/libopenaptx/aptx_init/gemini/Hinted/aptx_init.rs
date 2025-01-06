
use std::mem::MaybeUninit;
use std::ptr::null_mut;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct aptx_filter_signal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; 16],
    inner_filter_signal: [[aptx_filter_signal; 16]; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 16],
    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; 16],
    invert_quantize: [aptx_invert_quantize; 16],
    prediction: [aptx_prediction; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

impl aptx_context {
    fn reset(&mut self) {
        let hd = self.hd;
        let mut i: usize;
        let mut chan: usize;
        let mut subband: usize;
        let mut channel: &mut aptx_channel;
        let mut prediction: &mut aptx_prediction;

        for i in 0..std::mem::size_of::<aptx_context>() {
            unsafe {
                ((self as *mut aptx_context as *mut u8).offset(i as isize))
                    .write_volatile(0);
            }
        }

        self.hd = hd;
        self.decode_skip_leading = 4;
        self.encode_remaining = 4;

        for chan in 0..NB_CHANNELS {
            channel = &mut self.channels[chan];
            for subband in 0..NB_SUBBANDS {
                prediction = &mut channel.prediction[subband];
                prediction.prev_sign[0] = 1;
                prediction.prev_sign[1] = 1;
            }
        }
    }

    fn init(hd: i32) -> Option<Box<aptx_context>> {
        let mut ctx: Box<aptx_context> = unsafe { MaybeUninit::uninit().assume_init() };
        ctx.hd = if hd != 0 { 1 } else { 0 };
        ctx.reset();
        Some(ctx)
    }
}

fn aptx_reset(ctx: &mut aptx_context) {
    ctx.reset();
}

fn aptx_init(hd: i32) -> Option<Box<aptx_context>> {
    aptx_context::init(hd)
}

const NB_CHANNELS: usize = 2;
const NB_SUBBANDS: usize = 16;
const LATENCY_SAMPLES: usize = 4;

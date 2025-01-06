
use std::mem::MaybeUninit;
use std::ptr::null_mut;

#[repr(C)]
pub struct aptx_filter_signal {
    pub buffer: [i32; 2 * 16],
    pub pos: u8,
}

#[repr(C)]
pub struct aptx_prediction {
    pub prev_sign: [i32; 2],
    pub s_weight: [i32; 2],
    pub d_weight: [i32; 24],
    pub pos: i32,
    pub reconstructed_differences: [i32; 48],
    pub previous_reconstructed_sample: i32,
    pub predicted_difference: i32,
    pub predicted_sample: i32,
}

#[repr(C)]
pub struct aptx_invert_quantize {
    pub quantization_factor: i32,
    pub factor_select: i32,
    pub reconstructed_difference: i32,
}

#[repr(C)]
pub struct aptx_quantize {
    pub quantized_sample: i32,
    pub quantized_sample_parity_change: i32,
    pub error: i32,
}

#[repr(C)]
pub struct aptx_QMF_analysis {
    pub outer_filter_signal: [aptx_filter_signal; 2],
    pub inner_filter_signal: [[aptx_filter_signal; 2]; 2],
}

#[repr(C)]
pub struct aptx_channel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; 4],

    pub qmf: aptx_QMF_analysis,
    pub quantize: [aptx_quantize; 4],
    pub invert_quantize: [aptx_invert_quantize; 4],
    pub prediction: [aptx_prediction; 4],
}

#[repr(C)]
pub struct aptx_context {
    pub decode_sync_packets: usize,
    pub decode_dropped: usize,
    pub channels: [aptx_channel; 2],
    pub hd: u8,
    pub sync_idx: u8,
    pub encode_remaining: u8,
    pub decode_skip_leading: u8,
    pub decode_sync_buffer_len: u8,
    pub decode_sync_buffer: [u8; 6],
}

pub fn aptx_reset(ctx: &mut aptx_context) {
    let hd = ctx.hd;
    let mut i: usize;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut aptx_channel;
    let mut prediction: &mut aptx_prediction;

    for i in 0..std::mem::size_of::<aptx_context>() {
        unsafe {
            *(((ctx as *mut aptx_context) as *mut u8).offset(i as isize)) = 0;
        }
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = (90 + 3) / 4;
    ctx.encode_remaining = (90 + 3) / 4;

    for chan in 0..2 {
        channel = &mut ctx.channels[chan];
        for subband in 0..4 {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

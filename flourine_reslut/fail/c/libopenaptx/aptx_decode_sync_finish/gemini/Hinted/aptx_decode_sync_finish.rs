
use std::mem::MaybeUninit;

#[repr(C)]
struct aptx_filter_signal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

#[repr(C)]
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
struct aptx_invert_quantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

#[repr(C)]
struct aptx_quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[repr(C)]
struct aptx_QMF_analysis {
    outer_filter_signal: [aptx_filter_signal; 4],
    inner_filter_signal: [[aptx_filter_signal; 4]; 4],
}

#[repr(C)]
struct aptx_channel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 12],
    qmf: aptx_QMF_analysis,
    quantize: [aptx_quantize; 12],
    invert_quantize: [aptx_invert_quantize; 12],
    prediction: [aptx_prediction; 12],
}

#[repr(C)]
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

extern "C" {
    fn aptx_reset(ctx: *mut aptx_context);
    fn aptx_decode_sync_finish(ctx: *mut aptx_context) -> usize;
}

impl aptx_context {
    fn reset(&mut self) {
        unsafe { aptx_reset(self) }
    }

    fn decode_sync_finish(&mut self) -> usize {
        unsafe { aptx_decode_sync_finish(self) }
    }
}

fn main() {
    let mut ctx = Box::new(MaybeUninit::<aptx_context>::uninit());
    let ctx = unsafe { ctx.assume_init_mut() };

    ctx.reset();
    let dropped = ctx.decode_sync_finish();
    println!("{}", dropped);
}

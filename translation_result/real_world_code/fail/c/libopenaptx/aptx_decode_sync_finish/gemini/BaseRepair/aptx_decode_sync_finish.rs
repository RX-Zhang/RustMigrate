
use std::mem::MaybeUninit;

#[repr(C)]
pub struct AptxFilterSignal {
    pub buffer: [i32; 2 * 16],
    pub pos: u8,
}

#[repr(C)]
pub struct AptxPrediction {
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
pub struct AptxInvertQuantize {
    pub quantization_factor: i32,
    pub factor_select: i32,
    pub reconstructed_difference: i32,
}

#[repr(C)]
pub struct AptxQuantize {
    pub quantized_sample: i32,
    pub quantized_sample_parity_change: i32,
    pub error: i32,
}

#[repr(C)]
pub struct AptxQmfAnalysis {
    pub outer_filter_signal: [AptxFilterSignal; 2],
    pub inner_filter_signal: [[AptxFilterSignal; 2]; 2],
}

#[repr(C)]
pub struct AptxChannel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; 4],
    pub qmf: AptxQmfAnalysis,
    pub quantize: [AptxQuantize; 4],
    pub invert_quantize: [AptxInvertQuantize; 4],
    pub prediction: [AptxPrediction; 4],
}

#[repr(C)]
pub struct AptxContext {
    pub decode_sync_packets: usize,
    pub decode_dropped: usize,
    pub channels: [AptxChannel; 2],
    pub hd: u8,
    pub sync_idx: u8,
    pub encode_remaining: u8,
    pub decode_skip_leading: u8,
    pub decode_sync_buffer_len: u8,
    pub decode_sync_buffer: [u8; 6],
}

pub fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize;
    let mut chan: usize;
    let mut subband: usize;
    let mut channel: &mut AptxChannel;
    let mut prediction: &mut AptxPrediction;

    for i in 0..std::mem::size_of::<AptxContext>() {
        unsafe {
            *(((ctx as *mut AptxContext) as *mut u8).offset(i as isize)) = 0;
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

pub fn aptx_decode_sync_finish(ctx: &mut AptxContext) -> usize {
    let dropped = ctx.decode_sync_buffer_len as usize;
    aptx_reset(ctx);
    dropped
}


use std::mem;

const LATENCY_LES: usize = 90;
const FTER_TS: usize = 6;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const NB_CHANNELS: usize = 2;

#[repr(C)]
struct AptxContext {
    hd: i32,
    decode_skip_leading: usize,
    encode_remaining: usize,
    channels: [Channel; NB_CHANNELS],
}

#[repr(C)]
struct Channel {
    prediction: [Prediction; NB_SUBBANDS],
}

#[repr(C)]
struct Prediction {
    prev_sign: [i16; 2],
}

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: usize = 0;
    let mut chan: usize = 0;
    let mut subband: usize = 0;
    let mut channel = &mut ctx.channels[0];
    let mut prediction = &mut channel.prediction[0];

    let ctx_ptr: *mut u8 = ctx as *mut _ as *mut u8;
    unsafe {
        for i in 0..mem::size_of::<AptxContext>() {
            *ctx_ptr.add(i) = 0;
        }
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = (LATENCY_LES + 3) / 4;
    ctx.encode_remaining = (LATENCY_LES + 3) / 4;

    for chan in 0..NB_CHANNELS {
        channel = &mut ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}



use std::mem;
use libc::{memset, c_void, size_t};

const LATENCY_SAMPLES: u32 = 90;
const FILTER_TAPS: u32 = 16;
const NB_FILTERS: u32 = 2;
const NB_SUBBANDS: u32 = 4;
const NB_CHANNELS: usize = 2;

// ... other code ...

#[no_mangle]
pub extern "C" fn aptx_reset_refactored(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    let mut i: u32 = 0;
    let mut chan: usize = 0;
    let mut subband: u32 = 0;
    let mut channel: &mut AptxChannel = &mut ctx.channels[0];
    let mut prediction: &mut AptxPrediction = &mut channel.prediction[0];

    unsafe {
        memset(
            ctx as *mut AptxContext as *mut c_void, // cast to c\_void
            0,
            mem::size_of::<AptxContext>() as size_t,
        );
    }

    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES + 3) as i16).wrapping_div(4);
    ctx.encode_remaining = ((LATENCY_SAMPLES.wrapping_add(3)) as i16).wrapping_div(4);

    for chan in 0..NB_CHANNELS {
        channel = &mut ctx.channels[chan];
        for subband in 0..NB_SUBBANDS {
            prediction = &mut channel.prediction[subband as usize];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

// ... other code ...

#[derive(Debug)]
pub struct AptxPrediction {
    pub prev_sign: [i16; 2],
    pub prev_val: [i16; 2],
    pub mem_val: [i16; FILTER_TAPS as usize],
}

#[derive(Debug)]
pub struct AptxChannel {
    pub prediction: [AptxPrediction; NB_SUBBANDS as usize],
    pub subband_energy: [i16; NB_SUBBANDS as usize],
}

#[derive(Debug)]
pub struct AptxContext {
    pub hd: i16,
    pub decode_skip_leading: i16,
    pub encode_remaining: i16,
    pub channels: [AptxChannel; NB_CHANNELS],
}


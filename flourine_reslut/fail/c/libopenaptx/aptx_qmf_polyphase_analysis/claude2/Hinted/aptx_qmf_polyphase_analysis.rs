
use std::{mem::MaybeUninit, num::Wrapping};

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1i32 << p) - 1;
    if (Wrapping(a) + Wrapping(1)).0 & !mask != 0 {
        (a >> 31) ^ mask
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(value as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; FILTER_TAPS], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..][..FILTER_TAPS];
    let mut e: i64 = 0;
    for i in 0..FILTER_TAPS {
        e += (sig[i] as i64) * (coeffs[i] as i64);
    }
    rshift64_clip24(e, shift)  
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos.wrapping_add(FILTER_TAPS as u8)) as usize] = sample;
    signal.pos = signal.pos.wrapping_add(1) & (FILTER_TAPS - 1) as u8;
}

fn aptx_qmf_polyphase_analysis(
    mut signals: [AptxFilterSignal; NB_FILTERS],
    coeffs: [[i32; FILTER_TAPS]; NB_FILTERS],
    shift: u32,
    samples: [i32; NB_FILTERS],
    low_subband_output: &mut MaybeUninit<i32>,
    high_subband_output: &mut MaybeUninit<i32>,
) {
    let mut subbands = [MaybeUninit::uninit(); NB_FILTERS];

    for i in 0..NB_FILTERS {
        aptx_qmf_filter_signal_push(
            &mut signals[NB_FILTERS - 1 - i],
            samples[i],
        );
        unsafe {
            subbands[i] = MaybeUninit::new(aptx_qmf_convolution(
                &signals[NB_FILTERS - 1 - i],
                &coeffs[NB_FILTERS - 1 - i],
                shift,
            ));
        }
    }

    unsafe {
        *low_subband_output = MaybeUninit::new(clip_intp2(
            subbands[0].assume_init() + subbands[1].assume_init(),
            23,
        ));
        *high_subband_output = MaybeUninit::new(clip_intp2(
            subbands[0].assume_init() - subbands[1].assume_init(),
            23,
        ));
    }
}


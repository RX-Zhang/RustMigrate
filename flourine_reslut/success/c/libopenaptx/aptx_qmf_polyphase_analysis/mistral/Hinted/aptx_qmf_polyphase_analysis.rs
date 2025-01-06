

use libc::uint8_t;
use libc::int32_t;
use libc::int64_t;
use std::boxed;

const NB_FILTERS: usize = 2;
const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: Box<[int32_t; 2 * FILTER_TAPS]>,
    pos: uint8_t,
}

fn clip_intp2(a: int32_t, p: u32) -> int32_t {
    if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: int64_t, shift: u32) -> int64_t {
    let rounding = (1 << (shift - 1)) as int64_t;
    let mask = ((1 << (shift + 1)) - 1) as int64_t;
    ((value + rounding) >> shift) - (if (value & mask) == rounding { 1 } else { 0 }) as int64_t
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[int32_t], shift: u32) -> int32_t {
    let sig = signal.buffer[signal.pos as usize];
    let mut e: int64_t = 0;
    let mut i: usize = 0;

    while i < FILTER_TAPS {
        e = e.wrapping_add((sig as int64_t) * (coeffs[i] as int64_t));
        i = i + 1;
    }

    (rshift64(e, shift) as i32) & 0x7ffffff
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: int32_t) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos as usize) + FILTER_TAPS] = sample;
    signal.pos = (signal.pos + 1) % (FILTER_TAPS as u8);
}

fn aptx_qmf_polyphase_analysis(
    signal: &mut [AptxFilterSignal],
    coeffs: &[[int32_t; FILTER_TAPS]],
    shift: u32,
    samples: &[int32_t],
    low_subband_output: &mut int32_t,
    high_subband_output: &mut int32_t,
) {
    let mut subbands: [int32_t; NB_FILTERS] = [0; NB_FILTERS];
    let mut i: usize = 0;

    while i < NB_FILTERS {
        aptx_qmf_filter_signal_push(&mut signal[i], samples[NB_FILTERS - 1 - i]);
        subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
        i = i + 1;
    }

    *low_subband_output = clip_intp2(
        subbands[0].wrapping_add(subbands[1]),
        (23 as u32),
    );
    *high_subband_output = clip_intp2(
        subbands[0].wrapping_sub(subbands[1]),
        (23 as u32),
    );
}


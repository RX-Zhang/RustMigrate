

use libc::int32_t;
use std::boxed::Box;
use std::mem;
use std::usize;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
 buffer: Box<[int32_t; 2 * FILTER_TAPS]>,
 pos: usize,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: int32_t) {
 signal.buffer[signal.pos] = sample;
 signal.buffer[signal.pos.wrapping_add(FILTER_TAPS)] = sample;
 signal.pos = (signal.pos + 1) & (FILTER_TAPS - 1);
}


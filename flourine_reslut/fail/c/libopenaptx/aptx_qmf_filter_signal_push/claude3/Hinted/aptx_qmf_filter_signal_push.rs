
use std::num::Wrapping;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[Wrapping<i32>; 2 * FILTER_TAPS]>,
    pos: Wrapping<usize>,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: Wrapping<i32>) {
    signal.buffer[signal.pos.0] = sample;
    signal.buffer[signal.pos.0 + FILTER_TAPS] = sample;
    signal.pos = Wrapping(signal.pos.0.wrapping_add(1) & (FILTER_TAPS - 1));
}

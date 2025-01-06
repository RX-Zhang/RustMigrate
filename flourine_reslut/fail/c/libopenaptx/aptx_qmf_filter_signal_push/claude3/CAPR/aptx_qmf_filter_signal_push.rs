
use std::mem;

const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: usize,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos = signal.pos;
    signal.buffer[pos] = sample;
    signal.buffer[pos.wrapping_add(FILTER_TAPS)] = sample;
    signal.pos = (pos + 1) & (FILTER_TAPS - 1);
}

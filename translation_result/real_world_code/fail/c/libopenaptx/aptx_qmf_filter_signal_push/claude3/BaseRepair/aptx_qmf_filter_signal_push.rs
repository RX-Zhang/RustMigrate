
use std::boxed::Box;

const FILTER_TAPS: usize = 16;

pub struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

pub fn new_aptx_filter_signal() -> AptxFilterSignal {
    AptxFilterSignal {
        buffer: Box::new([0; 2 * FILTER_TAPS]),
        pos: 0,
    }
}

pub fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[signal.pos as usize + FILTER_TAPS] = sample;
    signal.pos = signal.pos.wrapping_add(1) % (FILTER_TAPS as u8);
}

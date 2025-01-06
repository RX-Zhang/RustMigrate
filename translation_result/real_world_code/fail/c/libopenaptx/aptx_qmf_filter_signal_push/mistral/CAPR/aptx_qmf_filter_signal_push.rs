
pub const FILTER_TAPS: usize = 16;

#[derive(Debug)]
pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: usize,
}

impl AptxFilterSignal {
    pub fn new() -> Self {
        Self {
            buffer: [0; 2 * FILTER_TAPS],
            pos: 0,
        }
    }
}

pub fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos] = sample;
    signal.buffer[signal.pos + FILTER_TAPS] = sample;
    signal.pos = (signal.pos.wrapping_add(1)) & (FILTER_TAPS - 1);
}

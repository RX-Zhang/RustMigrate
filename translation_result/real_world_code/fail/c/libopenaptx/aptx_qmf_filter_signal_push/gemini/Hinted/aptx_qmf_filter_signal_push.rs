

const FILTER_TAPS: usize = 16;

#[derive(Debug, Clone)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: usize,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos] = sample;
    signal.buffer[signal.pos.wrapping_add(FILTER_TAPS)] = sample;
    signal.pos = signal.pos.wrapping_add(1) % FILTER_TAPS;
}

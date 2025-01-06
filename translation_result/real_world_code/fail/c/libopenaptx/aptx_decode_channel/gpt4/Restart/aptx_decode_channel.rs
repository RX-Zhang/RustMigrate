
const FILTER_TAPS: usize = 8; // Assuming a constant value for FILTER_TAPS

struct AptxFilterSignal {
    buffer: [i32; FILTER_TAPS * 2],
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[signal.pos as usize + FILTER_TAPS] = sample;
    signal.pos = (signal.pos.wrapping_add(1)) & (FILTER_TAPS as u8 - 1);
}

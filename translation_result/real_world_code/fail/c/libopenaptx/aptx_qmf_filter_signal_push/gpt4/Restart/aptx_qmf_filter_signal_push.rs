
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    let pos_usize = signal.pos as usize;
    signal.buffer[pos_usize] = sample;
    signal.buffer[pos_usize + FILTER_TAPS] = sample;
    signal.pos = ((signal.pos as usize).wrapping_add(1) & (FILTER_TAPS - 1)) as u8;
}

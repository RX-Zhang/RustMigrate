
const FILTER_TAPS: usize = 16;

struct AptxFilterSignal {
    buffer: Box<[i32; 2 * FILTER_TAPS]>,
    pos: u8,
}

impl AptxFilterSignal {
    fn new() -> Self {
        AptxFilterSignal {
            buffer: Box::new([0; 2 * FILTER_TAPS]),
            pos: 0,
        }
    }

    fn aptx_qmf_filter_signal_push(&mut self, sample: i32) {
        let pos_usize = self.pos as usize;
        self.buffer[pos_usize] = sample;
        self.buffer[pos_usize + FILTER_TAPS] = sample;
        self.pos = self.pos.wrapping_add(1) & (FILTER_TAPS as u8 - 1);
    }
}

fn main() {
    // Example usage
    let mut signal = AptxFilterSignal::new();
    signal.aptx_qmf_filter_signal_push(123);
}

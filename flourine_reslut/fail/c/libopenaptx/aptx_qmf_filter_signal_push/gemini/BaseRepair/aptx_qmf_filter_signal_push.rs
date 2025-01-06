
use std::ops::{Add, AddAssign};

const FILTER_TAPS: usize = 16;

#[derive(Debug)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

impl AptxFilterSignal {
    fn push(&mut self, sample: i32) {
        self.buffer[self.pos as usize] = sample;
        self.buffer[(self.pos + FILTER_TAPS as u8) as usize] = sample;
        self.pos = (self.pos + 1) & (FILTER_TAPS as u8 - 1);
    }
}


use std::ops::{Add, AddAssign};

#[derive(Default)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

impl AptxFilterSignal {
    fn push(&mut self, sample: i32) {
        self.buffer[self.pos as usize] = sample;
        self.buffer[(self.pos + 16) as usize] = sample;
        self.pos = self.pos.wrapping_add(1) & (16 - 1);
    }
}

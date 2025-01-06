
use std::ops::{Index, IndexMut};

const FILTER_TAPS: usize = 16;

pub struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u8,
}

impl AptxFilterSignal {
    pub fn push(&mut self, sample: i32) {
        self.buffer[self.pos as usize] = sample;
        self.buffer[(self.pos as usize + FILTER_TAPS) % (2 * FILTER_TAPS)] = sample;
        self.pos = (self.pos + 1) & (FILTER_TAPS - 1) as u8;
    }
}

impl Index<usize> for AptxFilterSignal {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for AptxFilterSignal {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer[index]
    }
}

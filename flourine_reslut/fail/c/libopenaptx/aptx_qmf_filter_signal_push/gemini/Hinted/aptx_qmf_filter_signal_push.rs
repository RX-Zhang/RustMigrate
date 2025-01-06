
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct AptxFilterSignal {
    buffer: Vec<i32>,
    pos: u8,
}

impl AptxFilterSignal {
    pub fn new() -> Self {
        Self {
            buffer: vec![0; 2 * 16],
            pos: 0,
        }
    }

    pub fn push(&mut self, sample: i32) {
        self.buffer[self.pos as usize] = sample;
        self.buffer[(self.pos + 16) as usize] = sample;
        self.pos = (self.pos + 1) & (16 - 1);
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


use std::ops::Index;
use std::ops::IndexMut;
use std::u32;

const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [i32; 2 * FILTER_TAPS],
    pos: u32,
}

impl AptxFilterSignal {
    fn new() -> Self {
        AptxFilterSignal {
            buffer: [0; 2 * FILTER_TAPS],
            pos: 0,
        }
    }
}

impl Index<usize> for AptxFilterSignal {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for AptxFilterSignal {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        &mut self.buffer[index]
    }
}

fn aptx_qmf_filter_signal_push(signal: &mut AptxFilterSignal, sample: i32) {
    signal.buffer[signal.pos as usize] = sample;
    signal.buffer[(signal.pos + FILTER_TAPS as u32) as usize] = sample;
    signal.pos = (signal.pos + 1) % (FILTER_TAPS as u32);
}

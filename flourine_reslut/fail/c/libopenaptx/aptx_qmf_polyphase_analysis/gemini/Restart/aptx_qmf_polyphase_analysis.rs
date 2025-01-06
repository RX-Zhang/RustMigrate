
use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

impl AptxFilterSignal {
    fn push(&mut self, sample: i32) {
        self.buffer[self.pos as usize] = sample;
        self.buffer[(self.pos + 16) as usize] = sample;
        self.pos = (self.pos + 1) & (16 - 1);
    }

    fn convolution(&self, coeffs: &[i32], shift: u32) -> i32 {
        let mut e = 0i64;
        for i in 0..16 {
            e += (self.buffer[i as usize] as i64) * (coeffs[i as usize] as i64);
        }
        ((e + (1 << (shift - 1))) >> shift) as i32 - ((e & ((1 << (shift + 1)) - 1)) == (1 << (shift - 1))) as i32
    }
}

fn aptx_qmf_polyphase_analysis(
    signal: &mut [AptxFilterSignal; 2],
    coeffs: &[[i32; 16]; 2],
    shift: u32,
    samples: &[i32; 2],
    low_subband_output: &mut i32,
    high_subband_output: &mut i32,
) {
    let mut subbands = [0i32; 2];
    for i in 0..2 {
        signal[i].push(samples[2 - 1 - i]);
        subbands[i] = signal[i].convolution(&coeffs[i], shift);
    }

    *low_subband_output = (subbands[0] + subbands[1]) >> 23;
    *high_subband_output = (subbands[0] - subbands[1]) >> 23;
}

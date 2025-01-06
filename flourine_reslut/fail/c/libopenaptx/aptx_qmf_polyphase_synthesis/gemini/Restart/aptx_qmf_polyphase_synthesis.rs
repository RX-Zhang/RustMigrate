
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
        let mut e: i64 = 0;
        let sig = &self.buffer[self.pos as usize..];
        for i in 0..16 {
            e += (sig[i] as i64) * (coeffs[i] as i64);
        }
        ((e + (1 << (shift - 1))) >> shift) as i32 - (((e & ((1 << (shift + 1)) - 1)) == (1 << (shift - 1))) as i32)
    }
}

fn aptx_qmf_polyphase_synthesis(
    signal: &mut [AptxFilterSignal; 2],
    coeffs: &[[i32; 16]; 2],
    shift: u32,
    low_subband_input: i32,
    high_subband_input: i32,
    samples: &mut [i32; 2],
) {
    let subbands = [low_subband_input.wrapping_add(high_subband_input), low_subband_input.wrapping_sub(high_subband_input)];
    for i in 0..2 {
        signal[i].push(subbands[1 - i]);
        samples[i] = signal[i].convolution(&coeffs[i], shift);
    }
}

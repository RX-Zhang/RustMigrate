
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
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
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32 + (1 << p)) & !(((2 << p) - 1) as u32)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32; 16], shift: u32) -> i32 {
    let sig = &signal.buffer[signal.pos as usize..];
    let mut e = 0i64;
    for i in 0..16 {
        e += (sig[i] as i64) * (coeffs[i] as i64);
    }
    rshift64_clip24(e, shift)
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
        subbands[i] = aptx_qmf_convolution(&signal[i], &coeffs[i], shift);
    }
    *low_subband_output = clip_intp2(subbands[0].wrapping_add(subbands[1]), 23);
    *high_subband_output = clip_intp2(subbands[0].wrapping_sub(subbands[1]), 23);
}

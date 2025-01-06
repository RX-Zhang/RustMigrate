

use std::mem;
use std::boxed;

const FILTER_TAPS: usize = 16;

#[repr(C)]
struct AptxFilterSignal {
buffer: Box<[i32; 2 * FILTER_TAPS]>,
pos: u8,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
if ((a as u32) + (1 << p)) & !((2 << p) - 1) != 0 {
return (a >> 31) ^ ((1 << p) - 1);
} else {
return a;
}
}

fn rshift64(value: i64, shift: u32) -> i64 {
let rounding = (1 << (shift - 1)) as i64;
let mask = ((1 << (shift + 1)) - 1) as i64;
((value + rounding) >> shift) - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
clip_intp2(rshift64(value, shift) as i32, 23)
}

fn aptx_qmf_convolution(signal: &AptxFilterSignal, coeffs: &[i32], shift: u32) -> i32 {
let sig = &signal.buffer[signal.pos as usize ..];
let mut e: i64 = 0;
for i in 0..FILTER_TAPS {
e = e.wrapping_add((sig[i] as i64) * (coeffs[i] as i64));
}
rshift64_clip24(e, shift)
}

fn main() {
let mut signal = AptxFilterSignal {
buffer: Box::new([0; 2 * FILTER_TAPS]),
pos: 0,
};
let coeffs: Box<[i32]> = Box::new([0; FILTER_TAPS]);
signal.buffer[0] = 5497553092608i64 as i32;
println!("{}", aptx_qmf_convolution(&signal, &coeffs, 0));
signal.buffer[0] = -2533377869609728i64 as i32;
println!("{}", aptx_qmf_convolution(&signal, &coeffs, 4292869938));
}


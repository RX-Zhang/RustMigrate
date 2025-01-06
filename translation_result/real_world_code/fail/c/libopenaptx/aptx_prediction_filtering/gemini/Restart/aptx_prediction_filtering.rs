
use std::ops::{Add, Sub};

const DIFFSIGN: fn(i32, i32) -> i32 = |x, y| ((x > y) as i32) - ((x < y) as i32);

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1 << p) - 1;
    if ((a as u32 + (1 << p)) & !(mask << 1)) > 0 {
        a.wrapping_add(1 << p)
    } else {
        a
    }
}

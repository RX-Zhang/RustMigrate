

use libc::int32_t;
use std::mem;
use std::boxed;

const DIFFSIGN: fn(i32, i32) -> i32 = {
|x, y| ((x > y) as i32) - ((x < y) as i32)
};

#[repr(C)]
struct AptxPrediction {
prev_sign: [i32; 2],
s_weight: [i32; 2],
d_weight: [i32; 24],
pos: i32,
reconstructed_differences: Box<[i32; 48]>,
previous_reconstructed_sample: i32,
predicted_difference: i32,
predicted_sample: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
let p_i32 = (p as i32) + a;
if p_i32 > 32767 {
32767
} else if p_i32 < -32768 {
-32768
} else {
p_i32
}
}


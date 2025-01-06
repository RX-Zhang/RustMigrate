
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a = a as u32;
    if (a + (1 << p)) & !(((2 << p) - 1)) != 0 {
        ((a >> 31) ^ ((1 << p) - 1)) as i32
    } else {
        a as i32
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1 << (shift - 1);
    let mask = (1 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - ((value & mask) == rounding) as i32
}

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    l_weight: [i32; 2],
    lmb_rms: [i32; 2],
    lmb_corr: [i32; 2],
}

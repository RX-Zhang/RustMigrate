
fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    let shifted = rshift32(value, shift);
    clip_intp2(shifted, 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    value.wrapping_shr(shift % 32)
}

fn clip_intp2(value: i32, bits: i32) -> i32 {
    let max_val = (1 << bits) - 1;
    let min_val = -(1 << bits);
    if value > max_val {
        max_val
    } else if value < min_val {
        min_val
    } else {
        value
    }
}

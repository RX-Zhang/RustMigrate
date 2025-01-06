
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    let rounding = (1 << (shift % 64 - 1)) as i64;
    let mask = ((1 << (shift % 64 + 1)) - 1) as i64;
    let shifted = ((value + rounding) >> (shift % 64)) - ((value & mask) == rounding) as i64;
    clip_intp2(shifted as i32, 23)
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << (p % 32)) & !(2 << (p % 32) - 1)) != 0 {
        (a >> 31) ^ ((1 << (p % 32)) - 1)
    } else {
        a
    }
}

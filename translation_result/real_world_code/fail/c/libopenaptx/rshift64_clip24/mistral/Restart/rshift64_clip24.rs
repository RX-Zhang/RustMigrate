
fn clip_intp2(a: i32, p: usize) -> i32 {
    if ((a as u32).wrapping_add((1u32 << p)) & !(((2u32 << p) - 1) as u32)) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = 1i64 << (shift as u32 - 1);
    let mask = (1i64 << (shift as u32 + 1)) - 1;
    ((value + rounding).wrapping_shr(shift as u32)) - ((value & mask) == rounding) as i64
}

fn rshift64_clip24(value: i64, shift: usize) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

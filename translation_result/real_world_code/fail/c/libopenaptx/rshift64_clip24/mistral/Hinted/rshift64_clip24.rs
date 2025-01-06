
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1u32 << p) & !(((2u32 << p) - 1u32))) != 0 {
        return (a >> 31) ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

pub fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1i64;
    ((value.wrapping_add(rounding)) >> shift) - ((value & mask) == rounding) as i64
}

pub fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2((rshift64(value, shift)) as i32, 23)
}

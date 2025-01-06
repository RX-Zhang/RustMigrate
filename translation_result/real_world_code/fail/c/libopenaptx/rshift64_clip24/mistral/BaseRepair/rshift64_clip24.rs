
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = (1 << p) - 1;
    let shifted = a + (1 << p);
    if (shifted & !(mask << 1)) != 0 {
        ((a >> 31) & !mask) | ((1 << p) - 1)
    } else {
        a
    }
}

pub fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1 << (shift - 1);
    let mask = ((1 << (shift + 1)) - 1) as i64;
    ((value.wrapping_add(rounding) >> shift) as i64) - ((value & mask) == rounding) as i64
}

pub fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

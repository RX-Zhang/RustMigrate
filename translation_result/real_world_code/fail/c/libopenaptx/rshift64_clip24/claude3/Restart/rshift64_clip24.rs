
#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a.wrapping_add(1 << p) as u32) & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)).wrapping_sub(1);
    ((value.wrapping_add(rounding)) >> shift).wrapping_sub((value & mask == rounding) as i64)
}

#[inline]
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

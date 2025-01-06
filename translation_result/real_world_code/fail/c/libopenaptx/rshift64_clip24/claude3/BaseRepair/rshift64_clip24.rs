
#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    let p = p & 31;  // Ensure p is in range 0..31
    if (a.wrapping_add(1 << p) as u32) & !((2u32 << p).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32 << p) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift64(value: i64, shift: u32) -> i64 {
    let shift = shift & 63;  // Ensure shift is in range 0..63
    if shift == 0 {
        return value;
    }
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << shift).wrapping_sub(1);
    value.wrapping_add(rounding).wrapping_shr(shift as u32).wrapping_sub((value & mask == rounding) as i64)
}

#[inline]
fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

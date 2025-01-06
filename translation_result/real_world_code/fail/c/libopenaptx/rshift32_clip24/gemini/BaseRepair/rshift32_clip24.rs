
#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    let p = p & 31;  // Ensure p is within 0-31 range
    if (a.wrapping_add(1 << p) as u32) & !((2u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

#[inline]
fn rshift32(value: i32, shift: u32) -> i32 {
    let shift = shift & 31;  // Ensure shift is within 0-31 range
    if shift == 0 {
        return value;
    }
    let rounding = 1i32.wrapping_shl(shift - 1);
    let mask = (1i32.wrapping_shl(shift + 1)).wrapping_sub(1);
    value.wrapping_add(rounding).wrapping_shr(shift)
        .wrapping_sub((value & mask == rounding) as i32)
}

#[inline]
fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

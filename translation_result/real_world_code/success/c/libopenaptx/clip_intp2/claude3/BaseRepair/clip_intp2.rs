
#[inline]
fn clip_intp2(a: i32, p: u32) -> i32 {
    let p = p % 32;  // Ensure p is within valid range for shifts
    if (a.wrapping_add(1i32.wrapping_shl(p)) as u32) & !((2u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ ((1i32.wrapping_shl(p)).wrapping_sub(1))
    } else {
        a
    }
}

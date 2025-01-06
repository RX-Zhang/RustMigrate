
#[inline]
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    let p = p % 32;  // Ensure p is within valid range for i32
    let mask = if p == 31 { i32::MAX } else { (1_i32 << p).wrapping_sub(1) };
    if (a.wrapping_add(1_i32.wrapping_shl(p)) as u32) & !((2_u32.wrapping_shl(p)).wrapping_sub(1)) != 0 {
        (a >> 31) ^ mask
    } else {
        a
    }
}

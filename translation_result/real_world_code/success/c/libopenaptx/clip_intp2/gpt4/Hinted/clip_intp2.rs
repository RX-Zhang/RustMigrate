
pub fn clip_intp2(a: i32, p: u32) -> i32 {
    if (((a as u32).wrapping_add(1u32.wrapping_shl(p))) & !((2u32.wrapping_shl(p)).wrapping_sub(1))) != 0 {
        return ((a >> 31) ^ ((1i32.wrapping_shl(p as u32)) - 1)) as i32;
    } else {
        return a;
    }
}

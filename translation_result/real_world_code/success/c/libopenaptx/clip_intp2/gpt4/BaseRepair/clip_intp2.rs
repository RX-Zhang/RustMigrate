
fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = ((1u32.wrapping_shl(p % 32)).wrapping_sub(1)) as i32;
    let shifted_a = (a as u32).wrapping_add(1u32.wrapping_shl(p % 32));
    if (shifted_a & !((1u32.wrapping_shl(p % 32)).wrapping_mul(2).wrapping_sub(1))) != 0 {
        (a >> 31) ^ mask
    } else {
        a
    }
}

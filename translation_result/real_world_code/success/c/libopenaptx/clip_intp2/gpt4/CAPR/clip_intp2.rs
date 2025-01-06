
fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_u32 = a as u32;
    let p_masked = p % 32; // Rust panics if shift exceeds the integer type's bit size
    let one_u32_shifted_p = 1u32.wrapping_shl(p_masked);
    let two_u32_shifted_p = 2u32.wrapping_shl(p_masked);
    
    if a_u32.wrapping_add(one_u32_shifted_p) & !(two_u32_shifted_p.wrapping_sub(1)) != 0 {
        a.wrapping_shr(31) ^ (one_u32_shifted_p.wrapping_sub(1) as i32)
    } else {
        a
    }
}


pub fn clip_intp2(a: i32, p: usize) -> i32 {
    let p = p % 32; // Ensure p is within the valid range for a shift operation
    let a_u32 = a as u32;
    let p_u32 = p as u32;
    let mask = !(((1u32 << p_u32) - 1) as u32);
    let shifted_a = a_u32.wrapping_add(1u32 << p_u32);
    if (shifted_a & mask) != 0 {
        return (a >> 31) as i32 ^ ((1 << p) - 1);
    } else {
        return a;
    }
}

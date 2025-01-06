
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (32 - bits % 32) % 32; // Ensure shift is within valid range
    let v = (val as u32).wrapping_shl(shift) as i32;
    v.wrapping_shr(shift)
}

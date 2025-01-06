
fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>() as u32).wrapping_sub(bits);
    let v = (val as u32).wrapping_shl(shift as u32);
    (v as i32).wrapping_shr(shift as u32)
}

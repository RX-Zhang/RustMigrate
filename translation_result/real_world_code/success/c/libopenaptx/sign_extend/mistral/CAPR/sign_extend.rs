
fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = (8 * std::mem::size_of::<i32>()) as usize - bits % 32;
    let v = (val as i32).wrapping_shl(shift as u32) as i32;
    (v.wrapping_shr(shift as u32)) as i32
}

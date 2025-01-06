
pub fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = (std::mem::size_of::<i32>() * 8) as usize - bits;
    let v = (val as u32) << shift;
    (v as i32).wrapping_shr(shift as u32)
}

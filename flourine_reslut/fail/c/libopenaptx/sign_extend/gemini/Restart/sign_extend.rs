
pub fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let mut v = i32::wrapping_shl((val as u32).try_into().unwrap(), shift) as i32;
    v = i32::wrapping_shr(v, shift);
    v
}

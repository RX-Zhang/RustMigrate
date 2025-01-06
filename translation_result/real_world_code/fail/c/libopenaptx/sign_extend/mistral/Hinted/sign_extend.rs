
pub fn sign_(val: i32, bits: usize) -> i32 {
    let shift = (std::mem::size_of::<i32>() * 8) - bits;
    let mut v = (val as u32) << shift;
    v = v.wrapping_shr(shift as u32);
    v as i32
}

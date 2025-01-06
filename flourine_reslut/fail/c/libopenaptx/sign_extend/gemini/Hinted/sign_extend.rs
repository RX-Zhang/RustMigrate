
pub fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let mut v = i32::from_be_bytes(val.to_be_bytes());
    v <<= shift;
    v >>= shift;
    v
}

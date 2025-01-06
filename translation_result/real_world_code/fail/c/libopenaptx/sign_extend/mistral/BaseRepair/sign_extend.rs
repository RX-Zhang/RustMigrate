
pub fn sign_extend(val: i32, bits: usize) -> i32 {
    if bits >= std::mem::size_of::<i32>() * 8 {
        return val;
    }
    let shift = (std::mem::size_of::<i32>() * 8 - bits) as u32;
    let v = (val as u32).wrapping_shl(shift) as i32;
    let mask = !((1 << (std::mem::size_of::<i32>() * 8 - bits)) - 1);
    (v & mask) >> shift
}

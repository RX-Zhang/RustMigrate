
use std::mem;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * mem::size_of::<i32>() as u32 - bits;
    let u = val as u32;
    let wrapped = u.wrapping_shl(shift);
    let result = wrapped as i32;
    result.wrapping_shr(shift as u32)
}

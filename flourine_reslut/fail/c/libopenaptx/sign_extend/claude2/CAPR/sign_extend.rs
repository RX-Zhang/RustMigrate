
use std::mem;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = (mem::size_of::<i32>() * 8) as u32 - bits;
    let mut v: i32 = (val as u32).wrapping_shl(shift) as i32;
    v = v.wrapping_shr(shift);
    v
}

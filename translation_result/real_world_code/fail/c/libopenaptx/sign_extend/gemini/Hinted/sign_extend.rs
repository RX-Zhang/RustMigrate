
use std::mem::transmute;

fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * std::mem::size_of::<i32>() as u32 - bits;
    let v: u32 = unsafe { transmute(val) };
    let v = v.wrapping_shl(shift as u32);
    let v: i32 = unsafe { transmute(v) };
    v.wrapping_shr(shift as u32)
}



use std::mem;
use std::u32;

fn sign_extend(val: i32, bits: usize) -> i32 {
    let shift = (mem::size_of::<i32>() as u32) * 8 - bits as u32;
    let mut v = (0i32, 0u32);
    v.1 = (val as u32) << shift;
    let extended_val = (v.1 >> shift) as i32;
    if (val as u32) >> (31 - shift) != 0 {
        extended_val | (-(extended_val >> 31)) as i32
    } else {
        extended_val
    }
}


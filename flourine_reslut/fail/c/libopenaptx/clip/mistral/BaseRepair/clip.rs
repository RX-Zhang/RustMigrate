

use std::cmp::min;
use std::cmp::max;
use std::os::raw::c_int;

fn clip(a: c_int, amin: c_int, amax: c_int) -> c_int {
    let a = a.wrapping_sub(amin).wrapping_add(1) as u32;
    let max_value = max(amin, amax) as u32;
    let a = a.min(u32::from(max_value) - a.wrapping_add(amin as u32 - 1) + 1) as c_int;
    a.wrapping_add(amin - 1)
}


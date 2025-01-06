
use std::num::Wrapping;

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = Wrapping(1i32.wrapping_shl(shift.wrapping_sub(1)));
    let mask = Wrapping(1i32.wrapping_shl(shift.wrapping_add(1))) - Wrapping(1);
    let shifted = (value + rounding.0) >> shift; 
    let masked = (value & mask.0) == rounding.0;
    shifted - masked as i32
}

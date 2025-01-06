
use std::primitive::i32;
use std::primitive::u32;

fn rshift32(value: i32, shift: u32) -> i32 {
    if shift > 31 {
        return 0;
    }
    let rounding = (1 << (shift - 1)) as i32;
    let mask = ((1 << (shift + 1)) - 1) as i32;
    let shifted = (value.wrapping_add(rounding)) >> shift;
    shifted.wrapping_sub(if (value & mask) == rounding { 1 } else { 0 })
}

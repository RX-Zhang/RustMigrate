
use std::num::Wrapping;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = Wrapping(1_i64);
    let mask = Wrapping(!0_i64);
    let shifted = (Wrapping(value) + rounding).0 >> shift;
    let masked = (((value & mask.0) == rounding.0) as i64);
    shifted - masked
}


use std::convert::TryInto;

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift % 64 - 1);
    let mask = (1i64 << (shift % 64 + 1)) - 1;
    ((value.wrapping_add(rounding)) >> (shift % 64)).wrapping_sub(((value & mask) == rounding) as i64)
}

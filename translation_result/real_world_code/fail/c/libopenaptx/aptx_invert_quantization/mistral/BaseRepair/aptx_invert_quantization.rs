
use std::i32;
use std::i64;
use std::mem;
use std::usize;
use std::boxed::Box;

// ... (other code remains the same)

const fn rshift64(value: i64, shift: usize) -> i64 {
    let rounding = (1 << (shift - 1)) as i64;
    let mask = ((1 << (shift + 1)) - 1) as i64;
    let shifted_value = (value + rounding) >> shift;
    shifted_value & !(mask << (64 - shift))
}

// ... (other code remains the same)

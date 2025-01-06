

use std::i32;

const TWO_PWR_32: i32 = 1 << 31;
const TWO_PWR_P_1: i32 = 1 << (32 - 3); // adjust the number 3 to the value of p

fn clip_intp2(mut a: i32, p: u32) -> i32 {
    const THRESHOLD: i32 = TWO_PWR_32 / 2;
    if a > THRESHOLD {
        a = THRESHOLD;
    } else if a < -THRESHOLD {
        a = -THRESHOLD;
    }
    a.wrapping_add(TWO_PWR_P_1.wrapping_neg()) >> p
}

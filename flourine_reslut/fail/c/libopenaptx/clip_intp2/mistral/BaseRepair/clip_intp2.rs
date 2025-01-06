

use std::u32;

fn clip_intp2(mut a: i32, p: u32) -> i32 {
const TWO_PWR_32: i32 = 1 << 31;
let TWO_PWR_P_1: i32 = (1 << (p - 1)) as i32;
if a > (TWO_PWR_32 - TWO_PWR_P_1) {
a = (TWO_PWR_32 - TWO_PWR_P_1);
} else if a < -(TWO_PWR_32 - TWO_PWR_P_1) {
a = -(TWO_PWR_32 - TWO_PWR_P_1);
}
a.wrapping_shr(p - 1)
}


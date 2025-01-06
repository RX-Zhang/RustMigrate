
use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let a_wrap = Wrapping(a as u32);
    let one = Wrapping(1u32);
    let two = Wrapping(2u32);
    let p_wrap = Wrapping(p);

    let mask = two.0.wrapping_shl(p_wrap.0) - 1;
    let threshold = one.0.wrapping_shl(p_wrap.0);

    if (a_wrap.0.wrapping_add(threshold) & !mask) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}


use std::num::Wrapping;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let a = Wrapping(a as u32);
    let p = 1u32 << p;
    let p = Wrapping(p);
    let max_val = Wrapping((!0u32).wrapping_shl(p.0));

    if a.0.wrapping_add(p.0) & !max_val.0 != 0 {
        (((a.0 as i32) >> 31) ^ ((p.0 - 1) as i32))
    } else {
        a.0 as i32
    }
}


use std::boxed::Box;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let uint_a = a as u32;
    let two_p = (1u32).wrapping_shl(p);
    let mask = two_p.wrapping_sub(1);
    
    if uint_a.wrapping_add(two_p) & !mask != 0 {
        (!(a >> 31) as u32 ^ mask) as i32
    } else {
        a
    }
}

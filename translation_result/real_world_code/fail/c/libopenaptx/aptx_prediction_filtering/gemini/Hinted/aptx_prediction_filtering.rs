
use std::i32;

fn clip_intp2(a: i32, p: u32) -> i32 {
    let mask = ((1 << (p + 1)) - 1) as i32;
    let value = ((a as u32) + (1 << p)) as i32;
    if (value & mask) != 0 {
        if a < 0 {
            (a >> p) - 1
        } else {
            (a >> p)
        }
    } else {
        a >> p
    }
}


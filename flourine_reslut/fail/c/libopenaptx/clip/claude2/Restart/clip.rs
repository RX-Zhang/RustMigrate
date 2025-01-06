
use std::cmp::Ordering;

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    match a.cmp(&amin) {
        Ordering::Less => amin,
        Ordering::Greater => {
            if a > amax {
                amax
            } else {
                a
            }
        }
        Ordering::Equal => a,
    }
}

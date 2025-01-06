
use std::cmp::Ordering;

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    match a.cmp(&amin) {
        Ordering::Less => amin,
        Ordering::Greater => {
            match a.cmp(&amax) {
                Ordering::Less => a,
                Ordering::Greater => amax,
                Ordering::Equal => amax,
            }
        }
        Ordering::Equal => a,
    }
}

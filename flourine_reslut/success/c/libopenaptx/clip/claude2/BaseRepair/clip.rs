
use std::{cmp::Ordering, num::Wrapping};

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    let a = Wrapping(a);
    let amin = Wrapping(amin); 
    let amax = Wrapping(amax);
    
    let result = match a.cmp(&amin) {
        Ordering::Less => amin,
        Ordering::Greater => {
            match a.cmp(&amax) {
                Ordering::Greater => amax,
                Ordering::Less | Ordering::Equal => a,
            }
        },
        Ordering::Equal | Ordering::Greater => a,
    };
    
    result.0
}

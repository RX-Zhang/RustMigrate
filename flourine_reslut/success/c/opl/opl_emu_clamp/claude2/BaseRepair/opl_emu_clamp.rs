
use std::cmp::Ordering;

fn opl_emu_clamp(value: i32, minval: i32, maxval: i32) -> i32 {
    match value.cmp(&minval) {
        Ordering::Less => minval,
        Ordering::Greater => match value.cmp(&maxval) {
            Ordering::Greater => maxval,
            Ordering::Less | Ordering::Equal => value,
        },
        Ordering::Equal | Ordering::Greater => value,
    }
}

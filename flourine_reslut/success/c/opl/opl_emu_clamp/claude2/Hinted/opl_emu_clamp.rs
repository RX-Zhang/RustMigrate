
use std::cmp::Ordering;

fn opl_emu_clamp(value: i32, minval: i32, maxval: i32) -> i32 {
    match value.cmp(&minval) {
        Ordering::Less => minval,
        _ if value > maxval => maxval,
        _ => value
    }
}

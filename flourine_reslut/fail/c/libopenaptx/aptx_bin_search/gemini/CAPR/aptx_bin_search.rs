
use std::cmp::Ordering;

pub fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> usize {
    let mut idx = 0;
    for i in (nb_intervals >> 1..0).rev() {
        match ((factor as i64) * (intervals[idx + i] as i64)).cmp(&((value as i64) << 24)) {
            Ordering::Less => idx += i,
            _ => (),
        }
    }
    idx
}


use std::cmp::Ordering;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx: usize = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64).wrapping_mul(intervals[idx.wrapping_add(i as usize) % intervals.len()] as i64) <= (value as i64).wrapping_shl(24) {
            idx = idx.wrapping_add(i as usize);
        }
        i = i.wrapping_shr(1);
    }

    idx as i32
}

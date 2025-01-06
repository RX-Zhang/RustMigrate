
use std::convert::TryInto;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx: i32 = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64 * intervals[idx.wrapping_add(i.try_into().unwrap()) as usize] as i64) <= ((value as i64) << 24) {
            idx = idx.wrapping_add(i);
        }
        i >>= 1;
    }

    idx
}


use std::convert::TryInto;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> usize {
    let mut idx: usize = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64).wrapping_mul(intervals[idx + i].try_into().unwrap()) <=
            ((value as i64) << 24)
        {
            idx += i;
        }
        i >>= 1;
    }
    idx
}

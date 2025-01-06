

use libc::int32_t;
use std::mem;

fn aptx_bin_search(value: int32_t, factor: int32_t,
                   intervals: &[int32_t], nb_intervals: usize)
                   -> int32_t {
    let mut idx = 0;
    let mut i = (nb_intervals as i64) >> 1;

    while i > 0 {
        i >>= 1;
        if (factor as i64) * (intervals[idx as usize + i as usize] as i64) <=
           ((value as i64) << 24) {
            idx = (idx as i64 + i as i64) as int32_t;
        }
    }

    idx
}


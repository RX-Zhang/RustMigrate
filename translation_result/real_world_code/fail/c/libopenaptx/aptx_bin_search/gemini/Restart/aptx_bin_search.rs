

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> i32 {
    let mut idx: i32 = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        let interval_value = factor.wrapping_mul(intervals[idx.wrapping_add(i as i32) as usize]);
        if (interval_value as i64) <= ((value as i64) << 24) {
            idx = idx.wrapping_add(i as i32);
        }
        i = i.wrapping_shr(1);
    }
    idx
}

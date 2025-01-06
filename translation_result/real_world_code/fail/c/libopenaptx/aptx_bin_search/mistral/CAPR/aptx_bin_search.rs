
fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> i32 {
    let mut idx: i32 = 0;
    let mut i: usize = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64 * intervals[idx as usize + i] as i64) <= ((value as i64) << 24) {
            idx = idx.wrapping_add(i as i32);
        }
        i >>= 1;
    }

    idx
}

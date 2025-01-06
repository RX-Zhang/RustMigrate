
pub fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> usize {
    let mut idx = 0;
    let mut i = (nb_intervals >> 1) as usize;

    while i > 0 {
        if (factor as i64 * intervals[idx as usize + i] as i64) <= ((value << 24) as i64) {
            idx = (idx as usize + i) as i32;
        }
        i >>= 1;
    }

    idx as usize
}


pub fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> i32 {
    let mut idx = 0;
    for i in (nb_intervals >> 1..0).rev() {
        if (factor as i64) * (intervals[idx + i] as i64) <= ((value as i64) << 24) {
            idx += i;
        }
    }
    idx.try_into().unwrap()
}

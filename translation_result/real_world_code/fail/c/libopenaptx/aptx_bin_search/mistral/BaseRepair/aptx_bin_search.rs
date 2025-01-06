

pub fn aptx_bin_searchvalue(value: i32, factor: i32, intervals: &[i32], nb_intervals: usize) -> i32 {
    let mut idx: usize = 0;
    for i in (0..(nb_intervals >> 1)).rev() {
        if (i as i64 * factor as i64 * (intervals[idx + i] as i64)) <= ((value as i64) << 24) {
            idx += i as usize;
        }
    }

    idx as i32
}


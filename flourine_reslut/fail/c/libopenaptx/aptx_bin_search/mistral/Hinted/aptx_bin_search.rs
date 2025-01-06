

use std::mem;
use std::ops::Shl;
use std::ptr;

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> i32 {
    let mut idx = 0;
    let mut i = (nb_intervals as f64).log2() as i32 as usize;

    while i > 0 {
        i = i.wrapping_shr(1);
        if (factor as i64) * (intervals[idx + i as usize] as i64) <= (value as i64) << 24 {
            idx = idx.wrapping_add(i);
        }
    }

    idx as i32
}

fn main() {
    let intervals: &[i32] = &[];
    let nb_intervals = 1513816064;

    let value = -2021152166;
    let factor = 8882055;

    let result = aptx_bin_search(value, factor, intervals, nb_intervals);
    println!("{}", result);

    let value = -2021161126;
    let factor = 8882055;

    let intervals: Box<[i32]> = Box::new([-2147483648; 1513816064]); // corrected this value

    let result = aptx_bin_search(value, factor, &intervals, nb_intervals);
    println!("{}", result);
}



fn aptx_bin_search(value: i32, factor: i32, intervals: &Box<[i32]>, nb_intervals: i32) -> i32 {
    let mut idx = 0i32;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64) * (intervals[(idx + i) as usize] as i64) <= ((value as i64) << 24) {
            idx = idx.wrapping_add(i);
        }
        i >>= 1;
    }

    idx
}

fn main() {
    let intervals = Box::new([1, 2, 3, 4, 5]) as Box<[i32]>;
    let nb_intervals = intervals.len() as i32;
    let value = 10;
    let factor = 2;

    let index = aptx_bin_search(value, factor, &intervals, nb_intervals);
    println!("Index: {}", index);
}

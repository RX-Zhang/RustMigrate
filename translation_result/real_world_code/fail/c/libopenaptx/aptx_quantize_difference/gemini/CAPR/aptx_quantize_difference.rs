
fn aptx_quantize_difference(
    quantize: &mut AptxQuantize,
    sample_difference: i32,
    dither: i32,
    quantization_factor: i32,
    tables: &AptxTables,
) {
    let intervals = &tables.quantize_intervals;
    let mut quantized_sample: usize;
    let mut dithered_sample: i64;
    let mut parity_change: usize;
    let mut d: i32;
    let mut mean: i32;
    let mut interval: i32;
    let mut inv: i32;
    let sample_difference_abs: i32 = sample_difference.abs();

    let sample_difference_abs = if sample_difference_abs > ((1 << 23) - 1) {
        (1 << 23) - 1
    } else {
        sample_difference_abs
    };

    quantized_sample = aptx_bin_search(
        sample_difference_abs >> 4,
        quantization_factor,
        intervals,
        tables.tables_size,
    );

    d = rshift32_clip24(
        (((dither as i64) * (dither as i64)) >> 32).try_into().unwrap(),
        7,
    ) - (1 << 23);
    d = rshift64(
        (d as i64) * (tables.quantize_dither_factors[quantized_sample] as i64),
        23,
    ) as i32;

    mean = (intervals[quantized_sample + 1] + intervals[quantized_sample]) / 2;
    interval = (intervals[quantized_sample + 1] - intervals[quantized_sample]) * ((if sample_difference < 0 { 0 } else { 1 }) as i32);

    dithered_sample =
        rshift64_clip24((dither as i64) * (interval as i64) + (((mean + d) as i64) << 32), 32) as i64;
    let error =
        ((sample_difference_abs as i64) << 20) - (dithered_sample * (quantization_factor as i64));
    quantize.error = rshift64(error, 23) as i32;
    quantize.error = quantize.error.abs();

    parity_change = quantized_sample;
    if error < 0 {
        quantized_sample -= 1;
    } else {
        parity_change -= 1;
    }

    inv = if sample_difference < 0 { 1 } else { 0 };
    quantize.quantized_sample = (quantized_sample as i32) ^ inv;
    quantize.quantized_sample_parity_change = (parity_change as i32) ^ inv;
}

#[derive(Debug, Clone, Copy)]
struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

#[derive(Debug, Clone, Copy)]
struct AptxTables {
    quantize_intervals: [i32; 128],
    quantize_dither_factors: [i32; 128],
    tables_size: usize,
}

fn aptx_bin_search(
    value: i32,
    quantization_factor: i32,
    intervals: &[i32],
    tables_size: usize,
) -> usize {
    let mut low: usize = 0;
    let mut high: usize = tables_size - 1;
    while low <= high {
        let mid = (low + high) / 2;
        let interval = intervals[mid];
        let next_interval = intervals[mid + 1];
        if interval <= value && next_interval > value {
            return mid;
        } else if value < interval {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    tables_size - 1
}

fn rshift32_clip24(mut value: i32, shift: u32) -> i32 {
    value = (value as i64 >> shift).try_into().unwrap();
    if value > (1 << 23) - 1 {
        (1 << 23) - 1
    } else if value < -(1 << 23) {
        -(1 << 23)
    } else {
        value
    }
}

fn rshift64_clip24(value: i64, shift: u32) -> i64 {
    let shifted_value = value >> shift;
    if shifted_value > ((1 << 23) - 1) as i64 {
        ((1 << 23) - 1) as i64
    } else if shifted_value < (-(1 << 23)) as i64 {
        (-(1 << 23)) as i64
    } else {
        shifted_value
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    value >> shift
}


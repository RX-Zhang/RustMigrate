
struct AptxTables {
    quantize_intervals: Box<[i32]>,
    quantize_dither_factors: Box<[i32]>,
    tables_size: usize,
}

struct AptxQuantize {
    error: i32,
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
}

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], size: usize) -> i32 {
    let mut low = 0;
    let mut high = size - 1;

    while low < high {
        let mid = (low + high) / 2;
        if (value as i64).wrapping_mul(factor as i64) < (intervals[mid] as i64) << 7 {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low as i32
}

fn clip_intp2(value: i32, bits: i32) -> i32 {
    let max = (1 << (bits - 1)) - 1;
    let min = -(1 << (bits - 1));
    value.max(min).min(max)
}

fn aptx_quantize_difference(quantize: &mut AptxQuantize, sample_difference: i32, dither: i32, quantization_factor: i32, tables: &AptxTables) {
    let intervals = &tables.quantize_intervals;
    let mut sample_difference_abs = sample_difference;
    if sample_difference_abs < 0 {
        sample_difference_abs = -sample_difference_abs;
    }
    sample_difference_abs = sample_difference_abs.min((1 << 23) - 1);

    let mut quantized_sample = aptx_bin_search(sample_difference_abs >> 4, quantization_factor, intervals, tables.tables_size);

    let d = (((dither as i64).wrapping_mul(dither as i64) >> 32) >> 7).wrapping_sub(1 << 23);
    let d = ((d as i64).wrapping_mul(tables.quantize_dither_factors[quantized_sample as usize] as i64) >> 23) as i32;

    let mean = (intervals[(quantized_sample + 1) as usize] + intervals[quantized_sample as usize]) / 2;
    let interval = (intervals[(quantized_sample + 1) as usize] - intervals[quantized_sample as usize]) * (if sample_difference < 0 { -1 } else { 1 });

    let dithered_sample = ((dither as i64).wrapping_mul(interval as i64) + ((clip_intp2(mean + d, 23) as i64) << 32)) >> 32;
    let error = ((sample_difference_abs as i64) << 20).wrapping_sub((dithered_sample as i64).wrapping_mul(quantization_factor as i64));
    quantize.error = (error >> 23) as i32;
    if quantize.error < 0 {
        quantize.error = -quantize.error;
    }

    let mut parity_change = quantized_sample;
    if error < 0 {
        quantized_sample -= 1;
    } else {
        parity_change -= 1;
    }

    let inv = if sample_difference < 0 { -1 } else { 0 };
    quantize.quantized_sample = quantized_sample ^ inv;
    quantize.quantized_sample_parity_change = parity_change ^ inv;
}

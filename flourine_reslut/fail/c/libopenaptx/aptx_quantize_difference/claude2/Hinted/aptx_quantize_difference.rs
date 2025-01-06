

use std::{mem::size_of, ops::BitXorAssign};

struct AptxTables {
    quantize_intervals: Box<[i32]>,
    invert_quantize_dither_factors: Box<[i32]>,
    quantize_dither_factors: Box<[i32]>,
    quantize_factor_select_offset: Box<[i16]>,
    tables_size: i32,
    factor_max: i32,
    prediction_order: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if ((a as u32).wrapping_add(1 << p) & !((2 << p) - 1)) != 0 {
        (a >> 31) ^ ((1 << p) - 1)
    } else {
        a
    }
}

fn rshift64(value: i64, shift: u32) -> i64 {
    let rounding = 1i64 << (shift - 1);
    let mask = (1i64 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i64)
}

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    clip_intp2(rshift64(value, shift) as i32, 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i32)
}

fn rshift32_clip24(value: i32, shift: u32) -> i32 {
    clip_intp2(rshift32(value, shift), 23)
}

fn aptx_bin_search(value: i32, factor: i32, intervals: &[i32], nb_intervals: i32) -> usize {
    let mut idx = 0;
    let mut i = nb_intervals >> 1;

    while i > 0 {
        if (factor as i64) * (intervals[idx + i as usize] as i64) <= ((value as i64) << 24) {
            idx += i as usize;
        }
        i >>= 1;
    }

    idx
}

fn aptx_quantize_difference(quantize: &mut AptxQuantize, sample_difference: i32, dither: i32,
    quantization_factor: i32, tables: &mut AptxTables) {

    let mut intervals = &mut tables.quantize_intervals;
    
    let mut quantized_sample: i32;
    let mut dithered_sample: i32;
    let mut parity_change: i32;
    
    let mut d: i32;
    let mean: i32;
    let interval: i32;
    let inv: i32;
    let mut sample_difference_abs = sample_difference;

    if sample_difference_abs < 0 {
        sample_difference_abs = -sample_difference_abs;
    }

    if sample_difference_abs > ((1i32 << 23) - 1) {
        sample_difference_abs = (1i32 << 23) - 1;
    }

    quantized_sample = aptx_bin_search(sample_difference_abs >> 4,
        quantization_factor, intervals, tables.tables_size) as i32;

    d = rshift32_clip24((dither as i64 * dither as i64 >> 32) as i32, 7) - (1i32 << 23);
    
    let intervals = &mut intervals[quantized_sample as usize..];
    mean = (intervals[1] + intervals[0]) / 2;
    interval = (intervals[1] - intervals[0]) * (if sample_difference < 0 {-1} else {1});

    d = rshift64((d as i64 * tables.quantize_dither_factors[quantized_sample as usize] as i64) >> 23, 23) as i32;

    dithered_sample = rshift64_clip24(((clip_intp2(mean + d, 23) as i64) << 32) + dither as i64 * interval as i64 >> 32, 32);
        
    let error = ((sample_difference_abs as i64) << 20) -
        (dithered_sample as i64 * quantization_factor as i64);

    quantize.error = rshift64(error, 23) as i32;

    if quantize.error < 0 {
        quantize.error = -quantize.error;
    }

    parity_change = quantized_sample;

    if error < 0 {
        quantized_sample -= 1;
    } else {
        parity_change -= 1;
    }

    inv = if sample_difference < 0 {-1} else {1};
    quantize.quantized_sample ^= inv;
    quantize.quantized_sample_parity_change ^= inv;
}


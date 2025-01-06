
const QUANTIZATION_FACTORS: [i32; 32] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
];

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    let result = (value >> shift) as i32;
    if result > 8388607 {
        8388607
    } else if result < -8388608 {
        -8388608
    } else {
        result
    }
}

fn rshift32(value: i32, shift: u32) -> i32 {
    value.wrapping_shr(shift)
}

fn clip(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

struct AptxInvertQuantize {
    reconstructed_difference: i32,
    quantization_factor: i32,
    factor_select: i32,
}

struct AptxTables {
    quantize_intervals: [i32; 256],
    invert_quantize_dither_factors: [i32; 256],
    quantize_factor_select_offset: [i32; 256],
    factor_max: i32,
}

fn aptx_invert_quantization(invert_quantize: &mut AptxInvertQuantize, quantized_sample: i32, dither: i32, tables: &AptxTables) {
    let mut qr;
    let mut idx;
    let mut shift;
    let mut factor_select;

    idx = (quantized_sample ^ ((quantized_sample < 0) as i32).wrapping_neg()).wrapping_add(1);
    qr = tables.quantize_intervals[idx as usize] / 2;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(((qr as i64 * (1i64 << 32)) + dither as i64 * tables.invert_quantize_dither_factors[idx as usize] as i64), 32);
    invert_quantize.reconstructed_difference = ((invert_quantize.quantization_factor as i64 * qr as i64) >> 19) as i32;

    factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(factor_select + (tables.quantize_factor_select_offset[idx as usize] as i32 * (1 << 15)), 15);
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    idx = (invert_quantize.factor_select & 0xFF) >> 3;
    shift = (tables.factor_max - invert_quantize.factor_select) >> 8;
    invert_quantize.quantization_factor = (QUANTIZATION_FACTORS[idx as usize] << 11) >> shift;
}

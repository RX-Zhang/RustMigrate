
#[derive(Debug)]
struct AptxInvertQuantize {
    reconstructed_difference: i32,
    quantization_factor: i32,
    factor_select: i32,
}

#[derive(Debug)]
struct AptxTables {
    quantize_intervals: Vec<i32>,
    invert_quantize_dither_factors: Vec<i32>,
    quantize_factor_select_offset: Vec<i32>,
    factor_max: i32,
}

const QUANTIZATION_FACTORS: [i32; 8] = [
    15147, 17945, 21365, 25567, 30798, 37315, 45480, 55799,
];

fn rshift32(value: i32, shift: u32) -> i32 {
    (value as i64).wrapping_shr(shift as u32) as i32
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

fn rshift64_clip24(value: i64, shift: u32) -> i32 {
    let result = value.wrapping_shr(shift);
    clip(result as i32, -(1 << 23), (1 << 23) - 1)
}

fn aptx_invert_quantization(
    invert_quantize: &mut AptxInvertQuantize,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    let mut qr: i32 = 0;
    let mut idx: usize = 0;
    let mut shift: u32 = 0;
    let mut factor_select: i32 = 0;

    idx = ((quantized_sample ^ (if quantized_sample < 0 { 1 } else { 0 })) + 1) as usize;
    qr = (tables.quantize_intervals[idx] / 2) as i32;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(
        ((qr as i64) * (1 << 32)) + (dither as i64) * (tables.invert_quantize_dither_factors[idx] as i64),
        32,
    );
    invert_quantize.reconstructed_difference = (invert_quantize.quantization_factor * qr) >> 19;

    /* update factor_select */
    factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(
        factor_select + (tables.quantize_factor_select_offset[idx] as i32 * (1 << 15)),
        15,
    );
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    /* update quantization factor */
    idx = ((invert_quantize.factor_select & 0xFF) >> 3) as usize;
    shift = ((tables.factor_max - invert_quantize.factor_select) >> 8) as u32;
    invert_quantize.quantization_factor = ((QUANTIZATION_FACTORS[idx] as i32) << 11) >> shift;
}

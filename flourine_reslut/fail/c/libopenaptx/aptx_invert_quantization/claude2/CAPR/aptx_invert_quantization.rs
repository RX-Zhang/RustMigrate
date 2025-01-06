

use std::convert::TryInto;

const QUANTIZATION_FACTORS: [i16; 32] = [
    2048, 2093, 2139, 2186, 2233, 2282, 2332, 2383,
    2435, 2489, 2543, 2599, 2656, 2714, 2774, 2834,
    2896, 2960, 3025, 3091, 3158, 3228, 3298, 3371,
    3444, 3520, 3597, 3676, 3756, 3838, 3922, 4008,
];

struct AptxTables {
    quantize_intervals: Box<[i32]>,
    invert_quantize_dither_factors: Box<[i32]>,
    quantize_dither_factors: Box<[i32]>,
    quantize_factor_select_offset: Box<[i16]>,
    tables_size: i32,
    factor_max: i32,
    prediction_order: i32,
}

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

fn clip_intp2(a: i32, p: u32) -> i32 {
    if (a as u32).wrapping_add(1 << p) & !((1 << (p + 1)) - 1) != 0 {
        ((a >> 31) ^ ((1 << p) - 1))
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
    clip_intp2(rshift64(value, shift).try_into().unwrap(), 23)
}

fn rshift32(value: i32, shift: u32) -> i32 {
    let rounding = 1i32 << (shift - 1);
    let mask = (1i32 << (shift + 1)) - 1;
    ((value + rounding) >> shift) - (((value & mask) == rounding) as i32)
}

fn clip(a: i32, amin: i32, amax: i32) -> i32 {
    if a < amin {
        amin
    } else if a > amax {
        amax
    } else {
        a
    }
}

fn aptx_invert_quantization(
    invert_quantize: &mut AptxInvertQuantize,
    quantized_sample: i32,
    dither: i32,
    tables: &AptxTables,
) {
    let mut qr = 0;
    let mut idx = 0;
    let mut shift = 0;
    let mut factor_select = 0;
    
    idx = (quantized_sample ^ ((quantized_sample < 0) as i32)) + 1;
    qr = *tables.quantize_intervals.get(idx as usize).unwrap() / 2;
    if quantized_sample < 0 {
        qr = -qr;
    }

    qr = rshift64_clip24(
        (qr as i64).wrapping_mul((1i64 << 32) as i64) + (dither as i64) * (*tables.invert_quantize_dither_factors.get(idx as usize).unwrap() as i64),
        32);
    invert_quantize.reconstructed_difference = ((invert_quantize.quantization_factor as i64 * qr as i64) >> 19) as i32;

    factor_select = 32620 * invert_quantize.factor_select;
    factor_select = rshift32(factor_select.wrapping_add((tables.quantize_factor_select_offset[idx as usize] as i32 * (1 << 15)) as i32), 15);
    invert_quantize.factor_select = clip(factor_select, 0, tables.factor_max);

    idx = (invert_quantize.factor_select & 0xFF) >> 3;
    shift = (tables.factor_max - invert_quantize.factor_select) >> 8;
    invert_quantize.quantization_factor = ((QUANTIZATION_FACTORS[idx as usize] as i32) << 11) >> shift;
}



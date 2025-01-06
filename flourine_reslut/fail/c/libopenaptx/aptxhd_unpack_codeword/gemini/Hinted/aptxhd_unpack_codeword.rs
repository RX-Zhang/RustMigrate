
use std::mem;

#[repr(C)]
pub struct aptx_filter_signal {
    pub buffer: [i32; 2 * 16],
    pub pos: u8,
}

#[repr(C)]
pub struct aptx_prediction {
    pub prev_sign: [i32; 2],
    pub s_weight: [i32; 2],
    pub d_weight: [i32; 24],
    pub pos: i32,
    pub reconstructed_differences: [i32; 48],
    pub previous_reconstructed_sample: i32,
    pub predicted_difference: i32,
    pub predicted_sample: i32,
}

#[repr(C)]
pub struct aptx_invert_quantize {
    pub quantization_factor: i32,
    pub factor_select: i32,
    pub reconstructed_difference: i32,
}

#[repr(C)]
pub struct aptx_quantize {
    pub quantized_sample: i32,
    pub quantized_sample_parity_change: i32,
    pub error: i32,
}

#[repr(C)]
pub struct aptx_QMF_analysis {
    pub outer_filter_signal: [aptx_filter_signal; 4],
    pub inner_filter_signal: [[aptx_filter_signal; 4]; 4],
}

#[repr(C)]
pub struct aptx_channel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; 4],

    pub qmf: aptx_QMF_analysis,
    pub quantize: [aptx_quantize; 4],
    pub invert_quantize: [aptx_invert_quantize; 4],
    pub prediction: [aptx_prediction; 4],
}

pub fn aptx_quantized_parity(channel: &aptx_channel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..4 {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

pub fn sign_extend(val: i32, bits: u32) -> i32 {
    let shift = 8 * mem::size_of::<i32>() as u32 - bits;
    let mut v = val as u32;
    v <<= shift;
    (v as i32) >> shift
}

pub fn aptxhd_unpack_codeword(channel: &mut aptx_channel, codeword: u32) {
    channel.quantize[0].quantized_sample = sign_extend((codeword >> 0) as i32, 9);
    channel.quantize[1].quantized_sample = sign_extend((codeword >> 9) as i32, 6);
    channel.quantize[2].quantized_sample = sign_extend((codeword >> 15) as i32, 4);
    channel.quantize[3].quantized_sample = sign_extend((codeword >> 19) as i32, 5);
    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !1)
        | aptx_quantized_parity(channel);
}

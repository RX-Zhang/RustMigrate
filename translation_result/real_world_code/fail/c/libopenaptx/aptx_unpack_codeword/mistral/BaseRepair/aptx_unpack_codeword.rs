
use std::mem;

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

type Int32 = i32;
type Uint8 = u8;

#[repr(C)]
struct AptxFilterSignal {
    buffer: [Int32; 2 * FILTER_TAPS],
    pos: Uint8,
}

// Struct definition for AptxChannel
struct AptxChannel {
    quantize: [AptxQuantize; NB_SUBBANDS],
    dither_parity: Int32,
}

// Struct definition for AptxQuantize
struct AptxQuantize {
    quantized_sample: Int32,
}

fn aptx_quantized_parity(channel: &AptxChannel) -> Int32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity ^= channel.quantize[subband].quantized_sample;
    }
    parity & 1
}

fn sign_extend(val: Int32, bits: usize) -> Int32 {
    let SHIFT = (mem::size_of::<Int32>() as i32 * 8 - bits as i32) % 32;
    let mut v = (val as u32) << SHIFT;
    v >>= SHIFT;
    v as Int32
}

// Function definition for aptx_unpack_codeword
fn aptx_unpack_codeword(channel: &mut AptxChannel, codeword: u16) {
    channel.quantize[0].quantized_sample = sign_extend(codeword as Int32 >> 0, 7);
    channel.quantize[1].quantized_sample = sign_extend(codeword as Int32 >> 7, 4);
    channel.quantize[2].quantized_sample = sign_extend(codeword as Int32 >> 11, 2);
    channel.quantize[3].quantized_sample = sign_extend(codeword as Int32 >> 13, 3);
    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !(1 as Int32))
        | (aptx_quantized_parity(channel) as Int32);
}


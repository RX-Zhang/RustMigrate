
#[derive(Debug)]
struct QuantizedSample {
    quantized_sample: i32,
}

#[derive(Debug)]
struct AptxChannel {
    quantize: [QuantizedSample; 4],
}

fn aptx_unpack_codeword(channel: &mut AptxChannel, codeword: u16) {
    channel.quantize[0].quantized_sample = sign_extend((codeword >> 0) as i32, 7);
    channel.quantize[1].quantized_sample = sign_extend((codeword >> 7) as i32, 4);
    channel.quantize[2].quantized_sample = sign_extend((codeword >> 11) as i32, 2);
    channel.quantize[3].quantized_sample = sign_extend((codeword >> 13) as i32, 3);
    channel.quantize[3].quantized_sample = (channel.quantize[3].quantized_sample & !1) | aptx_quantized_parity(channel);
}

fn sign_extend(value: i32, bits: u32) -> i32 {
    let mask = (1 << bits) - 1;
    if value & (1 << (bits - 1)) != 0 {
        value | !mask
    } else {
        value & mask
    }
}

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let sum: i32 = channel.quantize.iter().map(|q| q.quantized_sample).sum();
    (sum % 2) as i32
}

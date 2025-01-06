
fn aptxhd_pack_codeword(channel: &AptxChannel) -> u32 {
    let parity = aptx_quantized_parity(channel);
    let mut result: u32 = 0;

    result |= (((channel.quantize[3].quantized_sample & 0x01E) as u32 | parity as u32) << 19) & 0xFFFFFFFF;
    result |= ((channel.quantize[2].quantized_sample & 0x00F) as u32) << 15;
    result |= ((channel.quantize[1].quantized_sample & 0x03F) as u32) << 9;
    result |= ((channel.quantize[0].quantized_sample & 0x1FF) as u32) << 0;

    result
}

struct AptxChannel {
    quantize: [AptxQuantizedSample; 4],
}

struct AptxQuantizedSample {
    quantized_sample: i32,
}

fn aptx_quantized_parity(channel: &AptxChannel) -> u32 {
    let mut parity: u32 = 0;
    for i in 0..4 {
        parity ^= channel.quantize[i].quantized_sample as u32;
    }
    parity & 0x01
}


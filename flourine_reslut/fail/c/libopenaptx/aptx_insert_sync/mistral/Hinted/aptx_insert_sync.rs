

use std::u8;

// ... (same as before)

struct AptxChannel {
data: u16,
}

fn aptx_quantized_parity(channel: &AptxChannel) -> u8 {
let quantized_data = (channel.data >> 7) & 1;
quantized_data as u8
}

fn aptx_check_parity(channels: &[AptxChannel; 2], sync_idx: &mut u8) -> bool {
let parity = (aptx_quantized_parity(&channels[0]) as u16)
.wrapping_add((((sync_idx.wrapping_add(1)) & 7) as u16).wrapping_neg())
.wrapping_add(1);
*sync_idx = sync_idx.wrapping_add(1) & 7;
parity as u8 == 1
}

// ... (rest of the code remains the same)


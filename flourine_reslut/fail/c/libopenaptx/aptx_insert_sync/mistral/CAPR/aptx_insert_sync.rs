

use std::u8;

#[derive(Copy, Clone)]
struct AptxChannel {
    dither_parity: i32,
    quantize: [Quantize; NB_SUBBANDS],
}

#[derive(Copy, Clone)]
struct Quantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

fn aptx_quantized_parity(channel: &AptxChannel) -> i32 {
    let mut parity = channel.dither_parity;
    for subband in 0..NB_SUBBANDS {
        parity = parity.wrapping_add(channel.quantize[subband].quantized_sample);
        parity = parity & 1;
    }
    parity
}

fn aptx_check_parity(channels: &[AptxChannel; 2], sync_idx: &mut u8) -> i32 {
    let parity = aptx_quantized_parity(&channels[0]) ^ aptx_quantized_parity(&channels[1]);
    let eighth = (*sync_idx == 7) as i32;
    *sync_idx = (*sync_idx + 1) & 7;
    parity ^ eighth
}

fn aptx_insert_sync(channels: &mut [AptxChannel; 2], sync_idx: &mut u8) {
    let mut min_error = i32::MAX;
    let mut min_index = 0;
    let mut c = &mut channels[1];
    let mut subband = 0;
    let map = [1, 2, 0, 3];

    if aptx_check_parity(channels, sync_idx) == 0 {
        let borrowed_channels: &mut [AptxChannel; 2] = channels;
        for i in 0..NB_SUBBANDS {
            if (&borrowed_channels[1].quantize[map[i]]).error < min_error {
                min_error = (&borrowed_channels[1].quantize[map[i]]).error;
                min_index = i;
            }
        }

        borrowed_channels[1].quantize[map[min_index]].quantized_sample =
            borrowed_channels[1].quantize[map[min_index]].quantized_sample_parity_change;
    }
}


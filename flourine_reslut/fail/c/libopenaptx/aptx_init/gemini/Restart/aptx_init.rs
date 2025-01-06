

use std::boxed::Box;

#[derive(Clone, Copy)]
enum Channels {
    Left,
    Right,
}

struct AptxFilterSignal {
    buffer: [i32; 2 * 16],
    pos: u8,
}

struct AptxPrediction {
    prev_sign: [i32; 2],
    s_weight: [i32; 2],
    d_weight: [i32; 24],
    pos: i32,
    reconstructed_differences: [i32; 48],
    previous_reconstructed_sample: i32,
    predicted_difference: i32,
    predicted_sample: i32,
}

struct AptxInvertQuantize {
    quantization_factor: i32,
    factor_select: i32,
    reconstructed_difference: i32,
}

struct AptxQuantize {
    quantized_sample: i32,
    quantized_sample_parity_change: i32,
    error: i32,
}

struct AptxQMFAnalysis {
    outer_filter_signal: [AptxFilterSignal; 2],
    inner_filter_signal: [[AptxFilterSignal; 2]; 2],
}

struct AptxChannel {
    codeword_history: i32,
    dither_parity: i32,
    dither: [i32; 4],
    qmf: AptxQMFAnalysis,
    quantize: [AptxQuantize; 4],
    invert_quantize: [AptxInvertQuantize; 4],
    prediction: [AptxPrediction; 4],
}

impl AptxChannel {
    fn default() -> Self {
        Self {
            codeword_history: 0,
            dither_parity: 0,
            dither: [0; 4],
            qmf: AptxQMFAnalysis {
                outer_filter_signal: [
                    AptxFilterSignal {
                        buffer: [0; 2 * 16],
                        pos: 0,
                    },
                    AptxFilterSignal {
                        buffer: [0; 2 * 16],
                        pos: 0,
                    },
                ],
                inner_filter_signal: [
                    [
                        AptxFilterSignal {
                            buffer: [0; 2 * 16],
                            pos: 0,
                        },
                        AptxFilterSignal {
                            buffer: [0; 2 * 16],
                            pos: 0,
                        },
                    ],
                    [
                        AptxFilterSignal {
                            buffer: [0; 2 * 16],
                            pos: 0,
                        },
                        AptxFilterSignal {
                            buffer: [0; 2 * 16],
                            pos: 0,
                        },
                    ],
                ],
            },
            quantize: [
                AptxQuantize {
                    quantized_sample: 0,
                    quantized_sample_parity_change: 0,
                    error: 0,
                },
                AptxQuantize {
                    quantized_sample: 0,
                    quantized_sample_parity_change: 0,
                    error: 0,
                },
                AptxQuantize {
                    quantized_sample: 0,
                    quantized_sample_parity_change: 0,
                    error: 0,
                },
                AptxQuantize {
                    quantized_sample: 0,
                    quantized_sample_parity_change: 0,
                    error: 0,
                },
            ],
            invert_quantize: [
                AptxInvertQuantize {
                    quantization_factor: 0,
                    factor_select: 0,
                    reconstructed_difference: 0,
                },
                AptxInvertQuantize {
                    quantization_factor: 0,
                    factor_select: 0,
                    reconstructed_difference: 0,
                },
                AptxInvertQuantize {
                    quantization_factor: 0,
                    factor_select: 0,
                    reconstructed_difference: 0,
                },
                AptxInvertQuantize {
                    quantization_factor: 0,
                    factor_select: 0,
                    reconstructed_difference: 0,
                },
            ],
            prediction: [
                AptxPrediction {
                    prev_sign: [1, 1],
                    s_weight: [0; 2],
                    d_weight: [0; 24],
                    pos: 0,
                    reconstructed_differences: [0; 48],
                    previous_reconstructed_sample: 0,
                    predicted_difference: 0,
                    predicted_sample: 0,
                },
                AptxPrediction {
                    prev_sign: [1, 1],
                    s_weight: [0; 2],
                    d_weight: [0; 24],
                    pos: 0,
                    reconstructed_differences: [0; 48],
                    previous_reconstructed_sample: 0,
                    predicted_difference: 0,
                    predicted_sample: 0,
                },
                AptxPrediction {
                    prev_sign: [1, 1],
                    s_weight: [0; 2],
                    d_weight: [0; 24],
                    pos: 0,
                    reconstructed_differences: [0; 48],
                    previous_reconstructed_sample: 0,
                    predicted_difference: 0,
                    predicted_sample: 0,
                },
                AptxPrediction {
                    prev_sign: [1, 1],
                    s_weight: [0; 2],
                    d_weight: [0; 24],
                    pos: 0,
                    reconstructed_differences: [0; 48],
                    previous_reconstructed_sample: 0,
                    predicted_difference: 0,
                    predicted_sample: 0,
                },
            ],
        }
    }
}

struct AptxContext {
    decode_sync_packets: usize,
    decode_dropped: usize,
    channels: [AptxChannel; 2],
    hd: bool,
    sync_idx: u8,
    encode_remaining: u8,
    decode_skip_leading: u8,
    decode_sync_buffer_len: u8,
    decode_sync_buffer: [u8; 6],
}

impl Default for AptxContext {
    fn default() -> Self {
        Self {
            decode_sync_packets: 0,
            decode_dropped: 0,
            channels: [AptxChannel::default(), AptxChannel::default()],
            hd: false,
            sync_idx: 0,
            encode_remaining: 0,
            decode_skip_leading: 0,
            decode_sync_buffer_len: 0,
            decode_sync_buffer: [0; 6],
        }
    }
}

fn aptx_reset(ctx: &mut AptxContext) {
    for i in 0..std::mem::size_of::<AptxContext>() {
        unsafe {
            *(((ctx as *mut AptxContext) as *mut u8).offset(i as isize)) = 0;
        }
    }
    ctx.hd = ctx.hd;
    ctx.decode_skip_leading = (90 + 3) / 4;
    ctx.encode_remaining = (90 + 3) / 4;
    for chan in 0..2 {
        let channel = &mut ctx.channels[chan];
        for subband in 0..4 {
            let prediction = &mut channel.prediction[subband];
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

fn aptx_init(hd: bool) -> Box<AptxContext> {
    let mut ctx = Box::new(AptxContext::default());
    aptx_reset(&mut *ctx);
    ctx
}


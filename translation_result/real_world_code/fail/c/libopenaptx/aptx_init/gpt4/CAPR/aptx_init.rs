
struct AptxContext {
    hd: i32,
    decode_skip_leading: u8,
    encode_remaining: u8,
    channels: Vec<Channel>,
}

impl Default for AptxContext {
    fn default() -> Self {
        AptxContext {
            hd: 0,
            decode_skip_leading: 0,
            encode_remaining: 0,
            channels: Vec::new(),
        }
    }
}

struct Channel {
    prediction: Vec<Prediction>,
}

struct Prediction {
    prev_sign: [i32; 2],
}

const LATENCY_SAMPLES: i32 = 100;

fn aptx_reset(ctx: &mut AptxContext) {
    let hd = ctx.hd;
    *ctx = AptxContext::default();
    ctx.hd = hd;
    ctx.decode_skip_leading = ((LATENCY_SAMPLES.wrapping_add(3)) / 4) as u8;
    ctx.encode_remaining = ((LATENCY_SAMPLES.wrapping_add(3)) / 4) as u8;

    for chan in &mut ctx.channels {
        for prediction in &mut chan.prediction {
            prediction.prev_sign[0] = 1;
            prediction.prev_sign[1] = 1;
        }
    }
}

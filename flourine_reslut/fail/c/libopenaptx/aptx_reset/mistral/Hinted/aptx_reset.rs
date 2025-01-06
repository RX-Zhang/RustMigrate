
use std::mem;

const NB_SUBBANDS: usize = 8;
const NB_CHANNELS: usize = 2;
const LATENCY_SAMPLES: u32 = 192;

// ... (the rest of the code remains the same)

#[derive(Default)]
struct AptxChannel {
    prediction: [[AptxPrediction; NB_SUBBANDS]; NB_CHANNELS],
}

#[derive(Default)]
struct AptxPrediction {
    prev_sign: [i16; 2],
}

#[derive(Default)]
struct AptxContext {
    hd: u32,
    // ... (the rest of the code remains the same)
}

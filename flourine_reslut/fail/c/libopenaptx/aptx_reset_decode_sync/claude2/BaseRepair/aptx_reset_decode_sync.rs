

use std::convert::TryInto;

pub const NB_CHANNELS: usize = 2;

pub struct AptxContext {
    pub decode_sync_packets: usize,
    pub decode_dropped: usize,
    pub channels: [Box<AptxChannel>; NB_CHANNELS],
    pub hd: bool,
    pub sync_idx: u8,
    pub encode_remaining: u8,
    pub decode_skip_leading: u8,
    pub decode_sync_buffer_len: u8,
    pub decode_sync_buffer: [u8; 6],  
}

pub const LATENCY_SAMPLES: usize = 90;
pub const NB_FILTERS: usize = 2;
pub const NB_SUBBANDS: usize = 4;
pub const FILTER_TAPS: usize = 16;

pub enum Channels {
    Left,  
    Right,
    NbChannels,
}

pub struct AptxFilterSignal {
    pub buffer: [i32; 2 * FILTER_TAPS],
    pub pos: u8,  
}

pub struct AptxPrediction {
    pub prev_sign: [i32; 2],
    pub s_weight: [i32; 2],
    pub d_weight: [i32; 24],  
    pub pos: i32,
    pub reconstructed_differences: [i32; 48],
    pub previous_reconstructed_sample: i32,
    pub predicted_difference: i32,
    pub predicted_sample: i32,
}

pub struct AptxInvertQuantize {
    pub quantization_factor: i32,
    pub factor_select: i32, 
    pub reconstructed_difference: i32,  
}

pub struct AptxQuantize {
    pub quantized_sample: i32,
    pub quantized_sample_parity_change: i32,
    pub error: i32,
}

pub struct AptxQmfAnalysis {
    pub outer_filter_signal: [AptxFilterSignal; NB_FILTERS],
    pub inner_filter_signal: [[AptxFilterSignal; NB_FILTERS]; NB_FILTERS],  
}

pub struct AptxChannel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; NB_SUBBANDS],
    pub qmf: AptxQmfAnalysis,
    pub quantize: [AptxQuantize; NB_SUBBANDS],
    pub invert_quantize: [AptxInvertQuantize; NB_SUBBANDS],
    pub prediction: [AptxPrediction; NB_SUBBANDS],
}

impl AptxContext {
    pub fn reset(&mut self) {
        self.decode_skip_leading = (LATENCY_SAMPLES + 3).wrapping_div(4).try_into().unwrap();
        self.encode_remaining = (LATENCY_SAMPLES + 3).wrapping_div(4).try_into().unwrap();

        for channel in &mut self.channels {
            for prediction in &mut channel.prediction {
                prediction.prev_sign = [1, 1];
            }
        }
    }

    pub fn reset_decode_sync(&mut self) {
        let decode_dropped = self.decode_dropped;
        let decode_sync_packets = self.decode_sync_packets;
        let decode_sync_buffer_len = self.decode_sync_buffer_len;
        let decode_sync_buffer = self.decode_sync_buffer.clone();

        self.reset();

        self.decode_sync_buffer
            .clone_from_slice(&decode_sync_buffer);
        self.decode_sync_buffer_len = decode_sync_buffer_len;
        self.decode_sync_packets = decode_sync_packets;
        self.decode_dropped = decode_dropped;
    }
}


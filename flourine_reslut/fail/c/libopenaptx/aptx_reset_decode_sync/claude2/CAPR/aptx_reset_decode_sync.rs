

use std::convert::TryInto;

pub struct AptxContext {
    pub decode_sync_packets: usize,
    pub decode_dropped: usize,
    pub channels: [Box<AptxChannel>; NB_CHANNELS],
    pub hd: u8,
    pub sync_idx: u8,
    pub encode_remaining: u8,
    pub decode_skip_leading: u8,
    pub decode_sync_buffer_len: u8,
    pub decode_sync_buffer: [u8; 6],
}

pub struct AptxChannel {
    pub codeword_history: i32,
    pub dither_parity: i32,
    pub dither: [i32; NB_SUBBANDS],
    pub qmf: Box<AptxQmfAnalysis>,
    pub quantize: [Box<AptxQuantize>; NB_SUBBANDS],
    pub invert_quantize: [Box<AptxInvertQuantize>; NB_SUBBANDS],
    pub prediction: [Box<AptxPrediction>; NB_SUBBANDS],
}

pub struct AptxQmfAnalysis {
    pub outer_filter_signal: [Box<AptxFilterSignal>; NB_FILTERS],
    pub inner_filter_signal: [[Box<AptxFilterSignal>; NB_FILTERS]; NB_FILTERS],
}

pub struct AptxFilterSignal {
    pub buffer: [i32; 2*FILTER_TAPS],
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

pub enum Channels {
    Left,
    Right,
    NbChannels,
}

pub const LATENCY_SAMPLES: usize = 90;
pub const NB_FILTERS: usize = 2;
pub const NB_SUBBANDS: usize = 4;
pub const FILTER_TAPS: usize = 16;
pub const NB_CHANNELS: usize = Channels::NbChannels as usize;

impl AptxContext {
    pub fn reset(&mut self) {
        for i in 0..std::mem::size_of::<Self>() {
            unsafe {
                *(&mut self.decode_sync_packets as *mut usize).wrapping_add(i) = 0;
            }
        }

        self.hd = 0;
        self.decode_skip_leading = ((LATENCY_SAMPLES + 3) / 4) as u8;
        self.encode_remaining = ((LATENCY_SAMPLES + 3) % 4) as u8;

        for chan in 0..NB_CHANNELS {
            let channel = unsafe { self.channels.get_unchecked_mut(chan) };
            for subband in 0..NB_SUBBANDS {
                let prediction = unsafe { channel.prediction.get_unchecked_mut(subband) };
                prediction.prev_sign[0] = 1;
                prediction.prev_sign[1] = 1;
            }
        }
    }

    pub fn reset_decode_sync(&mut self) {
        let decode_dropped = self.decode_dropped;
        let decode_sync_packets = self.decode_sync_packets;
        let decode_sync_buffer_len = self.decode_sync_buffer_len;
        let mut decode_sync_buffer = [0u8; 6];
        decode_sync_buffer.copy_from_slice(&self.decode_sync_buffer[..6]);

        self.reset();

        self.decode_sync_buffer[..6].copy_from_slice(&decode_sync_buffer);
        self.decode_sync_buffer_len = decode_sync_buffer_len;
        self.decode_sync_packets = decode_sync_packets;
        self.decode_dropped = decode_dropped;
    }
}



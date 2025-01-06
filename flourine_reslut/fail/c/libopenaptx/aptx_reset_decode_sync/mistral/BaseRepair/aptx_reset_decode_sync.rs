

use std::mem;

const LATENCY_SAMPLES: u32 = 90;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;
const FILTER_TAPS: usize = 16;

#[repr(C)]
pub struct AptxContext {
    decode_sync_packets: u32,
    decode_dropped: u32,
    channels: [Channel; 2],
    hd: u32,
    sync_idx: u32,
    encode_remaining: u32,
    decode_skip_leading: u32,
    decode_sync_buffer_len: u32,
    decode_sync_buffer: [u8; 6],
}

#[repr(C)]
pub struct Channel {
    prediction: [SubbandPrediction; 4],
}

#[repr(C)]
pub struct SubbandPrediction {
    prev_sign: [i16; 2],
}

impl AptxContext {
    pub fn new() -> AptxContext {
        AptxContext {
            decode_sync_packets: 0,
            decode_dropped: 0,
            channels: unsafe { mem::zeroed() },
            hd: 0,
            sync_idx: 0,
            encode_remaining: 0,
            decode_skip_leading: (LATENCY_SAMPLES.wrapping_add(3)).wrapping_div(4),
            decode_sync_buffer_len: 0,
            decode_sync_buffer: unsafe { mem::zeroed() },
        }
    }

    pub fn aptx_reset(&mut self) {
        unsafe {
            memset(
                self as *mut AptxContext as *mut libc::c_void,
                0,
                mem::size_of::<AptxContext>(),
            );
        }

        self.hd = self.hd;
        self.decode_skip_leading = (LATENCY_SAMPLES.wrapping_add(3)).wrapping_div(4);
        self.encode_remaining = (LATENCY_SAMPLES).wrapping_add(3).wrapping_div(4);

        for chan in self.channels.iter_mut() {
            for subband in 0..NB_SUBBANDS {
                let prediction = &mut chan.prediction[subband];
                prediction.prev_sign[0] = 1;
                prediction.prev_sign[1] = 1;
            }
        }
    }

    pub fn aptx_reset_decode_sync(&mut self) {
        let decode_dropped = self.decode_dropped;
        let decode_sync_packets = self.decode_sync_packets;
        let decode_sync_buffer_len = self.decode_sync_buffer_len;
        let mut decode_sync_buffer = [0u8; 6];
        let mut i = 0;

        for i in 0..6 {
            decode_sync_buffer[i] = self.decode_sync_buffer[i];
        }

        self.aptx_reset();

        for i in 0..6 {
            self.decode_sync_buffer[i] = decode_sync_buffer[i];
        }

        self.decode_sync_buffer_len = decode_sync_buffer_len;
        self.decode_sync_packets = decode_sync_packets;
        self.decode_dropped = decode_dropped;
    }
}

extern "C" {
    fn memset(dest: *mut libc::c_void, c: libc::c_int, n: libc::size_t);
}


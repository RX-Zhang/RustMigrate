

use std::mem;
use std::u8;

const LATENCY_SAMPLES: u8 = 90;
const FILTER_TAPS: usize = 16;
const NB_FILTERS: usize = 2;
const NB_SUBBANDS: usize = 4;

#[repr(u8)]
enum Channels {
    LEFT = 0,
    RIGHT,
}

const CHANNELS: [Channels; 2] = [Channels::LEFT, Channels::RIGHT];

fn wrap_add(a: u8, b: u8) -> u8 {
    u8::wrapping_add(a, b)
}

fn main() {}


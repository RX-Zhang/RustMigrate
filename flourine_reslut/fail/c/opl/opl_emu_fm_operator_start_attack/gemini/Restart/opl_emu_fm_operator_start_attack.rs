
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
enum EnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

#[derive(Debug)]
struct Registers {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

#[derive(Debug)]
struct OpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

#[derive(Debug)]
struct FmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: EnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OpdataCache,
    regs: Box<Registers>,
}

impl FmOperator {
    fn start_attack(&mut self) {
        if self.env_state == EnvelopeState::Attack {
            return;
        }
        self.env_state = EnvelopeState::Attack;
        self.phase = 0;
        if self.cache.eg_rate[EnvelopeState::Attack as usize] >= 62 {
            self.env_attenuation = 0;
        }
    }
}

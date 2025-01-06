
use std::boxed::Box;

#[derive(Copy, Clone)]
pub enum EnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

pub struct Registers {
    pub lfo_am_counter: u16,
    pub lfo_pm_counter: u16,
    pub noise_lfsr: u32,
    pub lfo_am: u8,
    pub regdata: Vec<u8>,
    pub waveform: Vec<Vec<u16>>,
}

pub struct OpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

pub struct FmOperator {
    pub choffs: u32,
    pub opoffs: u32,
    pub phase: u32,
    pub env_attenuation: u16,
    pub env_state: EnvelopeState,
    pub key_state: u8,
    pub keyon_live: u8,
    pub cache: OpdataCache,
    pub regs: Box<Registers>,
}

impl FmOperator {
    pub fn reset(&mut self) {
        self.phase = 0;
        self.env_attenuation = 0x3ff;
        self.env_state = EnvelopeState::Release;
        self.key_state = 0;
        self.keyon_live = 0;
    }
}

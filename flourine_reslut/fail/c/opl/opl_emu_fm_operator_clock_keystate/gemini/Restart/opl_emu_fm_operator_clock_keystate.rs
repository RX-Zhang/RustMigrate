

use std::boxed::Box;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum EnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

struct Registers {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

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
    fn start_release(&mut self) {
        if self.env_state >= EnvelopeState::Release {
            return;
        }
        self.env_state = EnvelopeState::Release;
    }

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

    fn clock_keystate(&mut self, keystate: u8) {
        if keystate != self.key_state {
            self.key_state = keystate;
            if keystate != 0 {
                self.start_attack();
            } else {
                self.start_release();
            }
        }
    }
}


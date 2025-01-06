
use std::boxed::Box;
use std::mem;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
}

struct OplEmuRegisters {
    lfo_am_counter: u16,
    lfo_pm_counter: u16,
    noise_lfsr: u32,
    lfo_am: u8,
    regdata: [u8; 0x200],
    waveform: [[u16; 0x400]; 8],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 6],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    choffs: u32,
    opoffs: u32,
    phase: u32,
    env_attenuation: u16,
    env_state: OplEmuEnvelopeState,
    key_state: u8,
    keyon_live: u8,
    cache: OplEmuOpdataCache,
    regs: Box<OplEmuRegisters>,
}

impl OplEmuFmOperator {
    fn start_release(&mut self) {
        if self.env_state >= OplEmuEnvelopeState::Release {
            return;
        }
        self.env_state = OplEmuEnvelopeState::Release;
    }

    fn start_attack(&mut self) {
        if self.env_state == OplEmuEnvelopeState::Attack {
            return;
        }
        self.env_state = OplEmuEnvelopeState::Attack;
        self.phase = 0;
        if self.cache.eg_rate[OplEmuEnvelopeState::Attack as usize] >= 62 {
            self.env_attenuation = 0;
        }
    }

    fn clock_keystate(&mut self, keystate: u32) {
        if keystate != self.key_state as u32 {
            self.key_state = keystate as u8;
            if keystate != 0 {
                self.start_attack();
            } else {
                self.start_release();
            }
        }
    }
}

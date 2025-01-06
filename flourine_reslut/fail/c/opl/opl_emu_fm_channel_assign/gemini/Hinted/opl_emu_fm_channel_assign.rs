
use std::boxed::Box;

#[derive(Clone, Copy)]
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

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
    op: [Box<OplEmuFmOperator>; 4],
    regs: Box<OplEmuRegisters>,
}

impl OplEmuFmOperator {
    fn set_choffs(&mut self, choffs: u32) {
        self.choffs = choffs;
    }
}

impl OplEmuFmChannel {
    fn assign(&mut self, index: usize, op: Box<OplEmuFmOperator>) {
        self.op[index] = op;
        if let Some(op) = self.op.get_mut(index) {
            op.set_choffs(self.choffs);
        }
    }
}

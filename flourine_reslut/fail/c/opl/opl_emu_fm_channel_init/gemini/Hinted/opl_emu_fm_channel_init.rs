
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

impl Default for OplEmuOpdataCache {
    fn default() -> Self {
        Self {
            phase_step: 0,
            total_level: 0,
            block_freq: 0,
            detune: 0,
            multiple: 0,
            eg_sustain: 0,
            eg_rate: [0; 6],
            eg_shift: 0,
        }
    }
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

impl Default for OplEmuFmOperator {
    fn default() -> Self {
        Self {
            choffs: 0,
            opoffs: 0,
            phase: 0,
            env_attenuation: 0,
            env_state: OplEmuEnvelopeState::Attack,
            key_state: 0,
            keyon_live: 0,
            cache: OplEmuOpdataCache::default(),
            regs: Box::new(OplEmuRegisters {
                lfo_am_counter: 0,
                lfo_pm_counter: 0,
                noise_lfsr: 0,
                lfo_am: 0,
                regdata: [0; 0x200],
                waveform: [[0; 0x400]; 8],
            }),
        }
    }
}

struct OplEmuFmChannel {
    choffs: u32,
    feedback: [i16; 2],
    feedback_in: i16,
    op: [Box<OplEmuFmOperator>; 4],
    regs: Box<OplEmuRegisters>,
}

impl OplEmuFmChannel {
    fn new(regs: Box<OplEmuRegisters>, choffs: u32) -> Self {
        Self {
            choffs,
            feedback: [0, 0],
            feedback_in: 0,
            op: [
                Box::new(OplEmuFmOperator::default()),
                Box::new(OplEmuFmOperator::default()),
                Box::new(OplEmuFmOperator::default()),
                Box::new(OplEmuFmOperator::default()),
            ],
            regs,
        }
    }
}

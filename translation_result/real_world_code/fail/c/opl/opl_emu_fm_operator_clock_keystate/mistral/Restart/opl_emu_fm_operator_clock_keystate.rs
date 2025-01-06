
use std::ops::{Shl, Shr};

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 0;
const OPL_EMU_REGISTERS_REGIST: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGIST],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
}

impl OplEmuOpdataCache {
    fn new() -> Self {
        OplEmuOpdataCache { phase_step: 0 }
    }

    fn set_phase_step(&mut self, value: u32) {
        self.phase_step = value & 0x1F;
    }

    fn get_phase_step(&self) -> u32 {
        self.phase_step
    }

    fn inc_phase_step(&mut self) {
        self.phase_step = (self.phase_step + 1) & 0x1F;
    }

    fn dec_phase_step(&mut self) {
        self.phase_step = (self.phase_step.wrapping_sub(1)) & 0x1F;
    }

    fn shift_phase_step_left(&mut self) {
        self.phase_step = self.phase_step.wrapping_shl(1);
    }

    fn shift_phase_step_right(&mut self) {
        self.phase_step = self.phase_step.wrapping_shr(1);
    }
}

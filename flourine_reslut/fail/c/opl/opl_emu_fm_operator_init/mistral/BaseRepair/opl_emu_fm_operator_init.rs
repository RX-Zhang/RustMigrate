

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy, Debug, PartialEq)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

impl Default for OplEmuEnvelopeState {
    fn default() -> Self {
        OplEmuEnvelopeState::OPL_EMU_EG_RELEASE
    }
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
}

impl Default for OplEmuRegisters {
    fn default() -> Self {
        OplEmuRegisters {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
        }
    }
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
        OplEmuOpdataCache {
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
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u32,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_regs: Box<OplEmuRegisters>,
    m_cache: OplEmuOpdataCache,
}

impl OplEmuFmOperator {
    fn new(regs: &mut OplEmuRegisters, opoffs: u32) {
        let fmop = OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: opoffs,
            m_phase: 0,
            m_env_attenuation: (1 << 10) - 1,
            m_env_state: Default::default(),
            m_key_state: 0,
            m_keyon_live: 0,
            m_regs: Box::new(mem::take(regs)).into(),
            m_cache: Default::default(),
        };
    }
}


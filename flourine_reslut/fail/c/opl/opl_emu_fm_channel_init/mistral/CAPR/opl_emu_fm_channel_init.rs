

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(u8)]
enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u8 = 6;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

#[repr(C)]
struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OPL_EMU_EG_STATES as usize],
    eg_shift: u8,
}

#[repr(C)]
struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
}

#[repr(C)]
struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Box<OplEmuFmOperator>; 4],
    m_regs: Box<OplEmuRegisters>,
}

impl OplEmuFmChannel {
    fn init(&mut self, choffs: u32) {
        self.m_choffs = choffs;
        self.m_feedback[0] = 0;
        self.m_feedback[1] = 0;
        self.m_feedback_in = 0;
        self.m_op[0] = Box::new(OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: 0,
            m_phase: 0,
            m_env_attenuation: 0,
            m_env_state: OplEmuEnvelopeState::OPL_EMU_EG_ATTACK,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: [0; OPL_EMU_EG_STATES as usize],
                eg_shift: 0,
            },
        });
        self.m_op[1] = Box::new(OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: 0,
            m_phase: 0,
            m_env_attenuation: 0,
            m_env_state: OplEmuEnvelopeState::OPL_EMU_EG_ATTACK,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: [0; OPL_EMU_EG_STATES as usize],
                eg_shift: 0,
            },
        });
        self.m_op[2] = Box::new(OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: 0,
            m_phase: 0,
            m_env_attenuation: 0,
            m_env_state: OplEmuEnvelopeState::OPL_EMU_EG_ATTACK,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: [0; OPL_EMU_EG_STATES as usize],
                eg_shift: 0,
            },
        });
        self.m_op[3] = Box::new(OplEmuFmOperator {
            m_choffs: 0,
            m_opoffs: 0,
            m_phase: 0,
            m_env_attenuation: 0,
            m_env_state: OplEmuEnvelopeState::OPL_EMU_EG_ATTACK,
            m_key_state: 0,
            m_keyon_live: 0,
            m_cache: OplEmuOpdataCache {
                phase_step: 0,
                total_level: 0,
                block_freq: 0,
                detune: 0,
                multiple: 0,
                eg_sustain: 0,
                eg_rate: [0; OPL_EMU_EG_STATES as usize],
                eg_shift: 0,
            },
        });
    }
}


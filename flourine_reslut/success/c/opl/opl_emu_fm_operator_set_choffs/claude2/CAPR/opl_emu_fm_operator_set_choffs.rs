
use std::boxed::Box;

pub const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
pub const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
pub const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

pub enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 6,
}

pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    pub m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32, 
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32, 
    pub eg_rate: [u8; OplEmuEnvelopeState::OPL_EMU_EG_STATES as usize],
    pub eg_shift: u8,
}

pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: OplEmuEnvelopeState,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
    pub m_regs: Box<OplEmuRegisters>,  
}

pub fn opl_emu_fm_operator_set_choffs(fmop: &mut OplEmuFmOperator, choffs: u32) {
    fmop.m_choffs = choffs;
}

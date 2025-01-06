

use std::boxed::Box;

pub const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
pub const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
pub const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone)]
pub struct OplEmuRegisters {
    pub m_lfo_am_counter: u16,
    pub m_lfo_pm_counter: u16,
    pub m_noise_lfsr: u32,
    pub m_lfo_am: u8,
    pub m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    pub m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

pub struct OplEmuOpdataCache {
    pub phase_step: u32,
    pub total_level: u32,
    pub block_freq: u32,
    pub detune: i32,
    pub multiple: u32,
    pub eg_sustain: u32,
    pub eg_rate: [u8; 6],
    pub eg_shift: u8,
}

pub struct OplEmuFmOperator {
    pub m_choffs: u32,
    pub m_opoffs: u32,
    pub m_phase: u32,
    pub m_env_attenuation: u16,
    pub m_env_state: u8,
    pub m_key_state: u8,
    pub m_keyon_live: u8,
    pub m_cache: OplEmuOpdataCache,
    pub m_regs: Box<OplEmuRegisters>,
}

pub fn opl_emu_fm_operator_init(fmop: &mut OplEmuFmOperator, regs: &OplEmuRegisters, opoffs: u32) {
    fmop.m_choffs = 0;
    fmop.m_opoffs = opoffs;
    fmop.m_phase = 0;
    fmop.m_env_attenuation = 0x3ff;
    fmop.m_env_state = 4;
    fmop.m_key_state = 0;
    fmop.m_keyon_live = 0;
    fmop.m_cache.eg_shift = 0;
    fmop.m_regs = Box::new(regs.clone());
}




use std::u16;
use std::u8;
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: u16 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u16 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u16 = 0x400;

#[derive(Copy, Clone, PartialEq)]
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
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; OplEmuEnvelopeState::States as usize],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    m_choffs: u32,
    m_opoffs: u32,
    m_phase: u32,
    m_env_attenuation: u16,
    m_env_state: OplEmuEnvelopeState,
    m_key_state: u8,
    m_keyon_live: u8,
    m_cache: OplEmuOpdataCache,
    m_regs: Box<OplEmuRegisters>,
}

fn opl_emu_fm_operator_start_attack(fmop: &mut OplEmuFmOperator) {
    if fmop.m_env_state == OplEmuEnvelopeState::Attack {
        return;
    }
    fmop.m_env_state = OplEmuEnvelopeState::Attack;
    fmop.m_phase = 0;
    if fmop.m_cache.eg_rate[OplEmuEnvelopeState::Attack as usize] >= 62 {
        fmop.m_env_attenuation = 0;
    }
}


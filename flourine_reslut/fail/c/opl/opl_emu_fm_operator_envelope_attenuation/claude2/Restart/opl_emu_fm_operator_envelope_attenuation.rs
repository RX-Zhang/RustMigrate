
use std::cmp;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

fn opl_min(a: u32, b: u32) -> u32 {
    if a < b {a} else {b}
}

enum OplEmuEnvelopeState {
    OplEmuEgAttack = 1,
    OplEmuEgDecay = 2,
    OplEmuEgSustain = 3,
    OplEmuEgRelease = 4,
    OplEmuEgStates = 6,  
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u16,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: Box<[u8; OplEmuEnvelopeState::OplEmuEgStates as usize]>,
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

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_op_lfo_am_enable(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(&regs, 0x20 + opoffs, 7, 1)
}

fn opl_emu_fm_operator_envelope_attenuation(fmop: &OplEmuFmOperator, am_offset: u16) -> u16 {
    let mut result = fmop.m_env_attenuation >> fmop.m_cache.eg_shift;

    if opl_emu_registers_op_lfo_am_enable(&fmop.m_regs, fmop.m_opoffs) != 0 {
        result = result.wrapping_add(am_offset as u16);
    }

    result = result.wrapping_add(fmop.m_cache.total_level);
    opl_min(result as u32, 0x3ff) as u16
}


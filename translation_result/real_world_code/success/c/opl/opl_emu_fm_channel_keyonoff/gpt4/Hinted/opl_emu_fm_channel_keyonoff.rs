
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[derive(Clone, Copy)]
enum OplEmuEnvelopeState {
    Attack = 1,
    Decay = 2,
    Sustain = 3,
    Release = 4,
    States = 6,
}

#[derive(Clone, Copy)]
enum OplEmuKeyonType {
    Normal = 0,
    Rhythm = 1,
    Csm = 2,
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEMS],
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

struct OplEmuFmChannel {
    m_choffs: u32,
    m_feedback: [i16; 2],
    m_feedback_in: i16,
    m_op: [Option<Box<OplEmuFmOperator>>; 4],
    m_regs: Box<OplEmuRegisters>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> (start % 32)) & ((1 << (length % 32)) - 1)
}

fn opl_emu_fm_operator_keyonoff(fmop: &mut OplEmuFmOperator, on: u32, type_: OplEmuKeyonType) {
    fmop.m_keyon_live = (fmop.m_keyon_live & !(1 << (type_ as u8))) | ((opl_emu_bitfield(on, 0, 1) as u8) << (type_ as u8));
}

fn opl_emu_fm_channel_keyonoff(fmch: &mut OplEmuFmChannel, states: u32, type_: OplEmuKeyonType, _chnum: u32) {
    for opnum in 0..fmch.m_op.len() {
        if let Some(fmop) = &mut fmch.m_op[opnum] {
            opl_emu_fm_operator_keyonoff(fmop, opl_emu_bitfield(states, opnum as i32, 1), type_);
        }
    }
}

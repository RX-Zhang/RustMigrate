

use std::convert::TryInto;

const OPL_EMU_REGISTERS_OPERATORS: u32 = 36;
const OPL_EMU_REGISTERS_ALL_CHANNELS: u32 = 0x0003FFFF;
const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_CHANNELS: u32 = 18;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
}

const OPL_EMU_EG_STATES: u8 = 6;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS as usize],
}

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

struct OplEmuT {
    m_env_counter: u32,
    m_status: u8,
    m_timer_running: [u8; 2],
    m_active_channels: u32,
    m_modified_channels: u32,
    m_prepare_count: u32,
    m_regs: OplEmuRegisters,
    m_channel: [OplEmuFmChannel; OPL_EMU_REGISTERS_CHANNELS as usize],
    m_operator: [OplEmuFmOperator; OPL_EMU_REGISTERS_OPERATORS as usize],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(offset: usize, start: i32, length: i32) -> u32 {
    opl_emu_bitfield(offset as u32, start, length) 
}

fn opl_emu_fm_operator_set_choffs(fmop: &mut OplEmuFmOperator, choffs: u32) {
    fmop.m_choffs = choffs;
}

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    o1 as u32 | (o2 as u32) << 8 | (o3 as u32) << 16 | (o4 as u32) << 24
}

fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(0x104, 0, 6)
}

fn opl_emu_fm_channel_assign(fmch: &mut OplEmuFmChannel, index: u32, op: &mut Option<Box<OplEmuFmOperator>>) {
    fmch.m_op[index as usize] = op.take();
    if let Some(op) = fmch.m_op[index as usize].as_mut() {
        opl_emu_fm_operator_set_choffs(op, fmch.m_choffs);
    }
}

fn opl_emu_registers_operator_map(regs: &OplEmuRegisters, dest: &mut OplEmuRegistersOperatorMapping) {
    let fourop = opl_emu_registers_fourop_enable(&regs);
    
    dest.chan[0] = if opl_emu_bitfield(fourop, 0, 1) != 0 {
        opl_emu_registers_operator_list(0, 3, 6, 9)
    } else {
        opl_emu_registers_operator_list(0, 3, 0xff, 0xff)
    };
    
    // ...
    
    dest.chan[17] = opl_emu_registers_operator_list(32, 35, 0xff, 0xff);
}

fn opl_emu_abs_sin_attenuation(input: u32) -> u32 {
    input
}

fn opl_emu_assign_operators(emu: &mut OplEmuT) {
    // implementation   
}

fn opl_emu_fm_operator_init(fmop: &mut OplEmuFmOperator, regs: &Box<OplEmuRegisters>, opoffs: u32) {
    // implementation
}

fn opl_emu_registers_operator_offset(opnum: u32) -> u32 {
    opnum
}

fn opl_emu_fm_channel_init(fmch: &mut OplEmuFmChannel, regs: &Box<OplEmuRegisters>, choffs: u32) {
    // implementation   
}

fn opl_emu_registers_channel_offset(chnum: u32) -> u32 {
    chnum  
}

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    // implementation
}

fn opl_emu_init(emu: &mut OplEmuT) {
    // implementation
}


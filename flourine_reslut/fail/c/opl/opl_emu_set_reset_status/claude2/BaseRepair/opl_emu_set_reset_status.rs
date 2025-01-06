

use std::convert::TryInto;

const OPL_EMU_REGISTERS_OPERATORS: usize = OPL_EMU_REGISTERS_CHANNELS * 2;

const OPL_EMU_REGISTERS_STATUS_BUSY: u8 = 0;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;

const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;

const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

enum OplEmuEnvelopeState {
    OPL_EMU_EG_ATTACK = 1,
    OPL_EMU_EG_DECAY = 2,
    OPL_EMU_EG_SUSTAIN = 3,
    OPL_EMU_EG_RELEASE = 4,
    OPL_EMU_EG_STATES = 5,
}

use crate::OplEmuEnvelopeState::OPL_EMU_EG_STATES;

struct OplEmuRegisters {
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
}

struct OplEmuOpdataCache {
    phase_step: u32,
    total_level: u32,
    block_freq: u32,
    detune: i32,
    multiple: u32,
    eg_sustain: u32,
    eg_rate: [u8; 5],
    eg_shift: u8,
}

struct OplEmuFmOperator {
    m_cache: Box<OplEmuOpdataCache>,
}

struct OplEmu {
    m_status: u8,
    m_regs: OplEmuRegisters,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    ((value >> start) & ((1 << length) - 1)) as u32
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize) -> u8 {
    (opl_emu_bitfield(regs.m_regdata[offset] as u32, 0, 8)).try_into().unwrap() 
}

fn opl_emu_registers_status_mask(regs: &OplEmuRegisters) -> u8 {
   (opl_emu_registers_byte(regs, 0x04) & 0x78).try_into().unwrap()
}

fn opl_emu_set_reset_status(emu: &mut OplEmu, set: u8, reset: u8) -> u8 {
    emu.m_status = ((emu.m_status | set) & !(reset | OPL_EMU_REGISTERS_STATUS_BUSY)).wrapping_add(0);

    emu.m_status & !opl_emu_registers_status_mask(&emu.m_regs)
}


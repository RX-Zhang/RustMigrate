

use std::convert::TryInto;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

struct OplEmuRegistersOperatorMapping {
    chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16, 
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &OplEmuRegisters, 
    offset: u32,
    start: u32,
    count: u32,
) -> u32 {
    let index = offset as usize;
    let value = regs.m_regdata[index] as u32;
    opl_emu_bitfield(value, start.try_into().unwrap(), count.try_into().unwrap()) 
}

fn opl_emu_registers_operator_list(o1: u8, o2: u8, o3: u8, o4: u8) -> u32 {
    o1 as u32 | (o2 as u32) << 8 | (o3 as u32) << 16 | (o4 as u32) << 24
}

fn opl_emu_registers_fourop_enable(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x104, 0, 6)  
}

fn opl_emu_registers_operator_map(
    regs: &OplEmuRegisters,
    dest: &mut OplEmuRegistersOperatorMapping,
) {
    // unchanged
}


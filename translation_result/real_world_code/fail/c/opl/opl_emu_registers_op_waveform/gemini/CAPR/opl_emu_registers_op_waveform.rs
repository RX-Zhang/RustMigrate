
use std::mem;

// Define constants
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

// Define the struct
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

// Function to extract a bitfield
fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    let mask = (1 << length.rem_euclid(32) as u32) - 1;
    (value.wrapping_shr(start.rem_euclid(32) as u32)) & mask
}

// Function to extract a byte from the registers
fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[(offset.wrapping_add(extra_offset) % OPL_EMU_REGISTERS_REGISTERS as u32) as usize] as u32, start as i32, count as i32)
}

// Function to get the new flag from the registers
fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

// Function to get the operator waveform from the registers
fn opl_emu_registers_op_waveform(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    let new_flag = opl_emu_registers_newflag(regs);
    let count = if new_flag != 0 { 3 } else { 2 };
    opl_emu_registers_byte(regs, 0xe0, 0, count, opoffs)
}

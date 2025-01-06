

use std::mem;
use libc::memset;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
m_lfo_am_counter: u16,
m_lfo_pm_counter: u16,
m_noise_lfsr: u32,
m_lfo_am: u8,
m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
(value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32) -> u8 {
let index = offset as usize / 2;
let shift = ((offset % 2) * 8 + start) as u32;
let mask = (1 << (8 - (start % 8) as u32)) - 1;
let value = (regs.m_regdata[index] as u16) << shift;
let result = (value & (1 << (8 + start)) as u16) as u8;
result | opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, 8 - (start % 8) as i32) as u8 & mask
}


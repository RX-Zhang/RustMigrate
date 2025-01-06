

use std::mem;

const OPL_EMU_REGISTERS_CHANNELS: usize = 18;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200; // changed to u32
const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;

#[repr(C)]
struct OplEmuRegistersOperatorMapping {
chan: [u32; OPL_EMU_REGISTERS_CHANNELS],
}

#[repr(C)]
struct OplEmuRegisters {
m_lfo_am_counter: u16,
m_lfo_pm_counter: u16,
m_noise_lfsr: u32,
m_lfo_am: u8,
m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS as usize]>, // changed to usize
m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
let start = start as u32;
let length = length as u32;
let mut result = 0;
result = value.wrapping_add( !( (1 << length) - 1 ) << start );
result = result >> (32 - start - length);
result
}


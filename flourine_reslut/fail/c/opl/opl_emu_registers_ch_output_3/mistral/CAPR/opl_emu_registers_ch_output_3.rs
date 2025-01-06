

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
m_lfo_am_counter: u16, // LFO AM counter
m_lfo_pm_counter: u16, // LFO PM counter
m_noise_lfsr: u32, // noise LFSR state
m_lfo_am: u8, // current LFO AM value
m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize], // register data
m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize], // waveforms
}

fn memset(dest: &mut [u8], c: u8, n: usize) {
for i in 0..n {
dest[i] = c;
}
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
let mut mask = (1 << length as u32) - 1;
mask <<= start as u32;
value & mask
}


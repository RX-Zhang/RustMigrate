

use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,               // LFO AM counter
    m_lfo_pm_counter: u16,               // LFO PM counter
    m_noise_lfsr: u32,                   // noise LFSR state
    m_lfo_am: u8,                        // current LFO AM value
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS as usize]>, // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize], // waveforms
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(u32::from(regs.m_regdata[(offset + extra_offset) as usize]), start as i32, count as i32)
}

fn opl_emu_registers_op_ksl(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    let temp = opl_emu_registers_byte(regs, 0x40, 6, 2, opoffs);
    (opl_emu_bitfield(temp, 1, 1) | (opl_emu_bitfield(temp, 0, 1) << 1)) as u32
}


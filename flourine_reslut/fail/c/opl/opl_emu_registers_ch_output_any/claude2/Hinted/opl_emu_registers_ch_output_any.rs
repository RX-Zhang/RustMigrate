

use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,              
    m_lfo_pm_counter: u16,            
    m_noise_lfsr: u32,                
    m_lfo_am: u8,                     
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS as usize]>,         
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize]>,
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start.wrapping_abs() as u32) & ((1 << length.wrapping_abs()) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0) 
}

fn opl_emu_registers_ch_output_any(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) > 0 {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 4, 4, 0)
    } else {
        1
    }
}


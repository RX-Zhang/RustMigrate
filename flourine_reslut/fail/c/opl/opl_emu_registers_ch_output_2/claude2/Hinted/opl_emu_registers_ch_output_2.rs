
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
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS as usize]>,    // register data
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize]>, // waveforms
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)  
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = offset as usize + extra_offset as usize;
    let value = regs.m_regdata[index] as u32;
    opl_emu_bitfield(value, start as i32, count as i32)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> bool {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0) != 0
}

fn opl_emu_registers_ch_output_2(regs: &OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 6, 1, 0)
    } else {
        0
    }
}


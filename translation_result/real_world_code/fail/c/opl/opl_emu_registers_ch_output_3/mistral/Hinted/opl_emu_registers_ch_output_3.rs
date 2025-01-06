
use std::usize;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct Registers {
    m_lfo_am_counter: u32,
    m_lfo_pm_counter: u32,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Box<[u8]>,
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>,
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &Registers, offset: usize, start: u32, count: u32, extra_offset: u32) -> u32 {
    let value = regs.m_regdata[offset + extra_offset as usize];
    opl_emu_bitfield(value as u32, start, count)
}

fn opl_emu_registers_newflag(regs: &Registers) -> bool {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0) != 0
}

fn opl_emu_registers_ch_output_3(regs: &Registers, choffs: usize) -> u32 {
    if opl_emu_registers_newflag(regs) {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 7, 1, 0)
    } else {
        0
    }
}

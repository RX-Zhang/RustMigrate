
use std::u16;
use std::u32;
use std::vec;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: Vec<u8>,
    m_waveform: Vec<Vec<u16>>,
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = offset as usize + extra_offset as usize;
    if index >= regs.m_regdata.len() {
        return 0; // or any other default value
    }
    opl_emu_bitfield(regs.m_regdata[index] as u32, start, count)
}

fn opl_emu_registers_op_lfo_am_enable(regs: &mut OplEmuRegisters, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 7, 1, opoffs)
}



use std::u16;
use std::u32;

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

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    if length > 32 || start > 31 {
        return 0;
    }
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let byte_offset = (offset + extra_offset) as usize;
    if byte_offset >= OPL_EMU_REGISTERS_REGISTERS as usize {
        return 0;
    }
    let value = regs.m_regdata[byte_offset] as u32;
    opl_emu_bitfield(value, start, count)
}

fn opl_emu_registers_newflag(regs: &OplEmuRegisters) -> u32 {
    match regs.m_regdata.get(0x105 as usize) {
        Some(val) => opl_emu_bitfield(*val as u32, 0, 1),
        None => 0,
    }
}

fn opl_emu_registers_op_waveform(regs: &OplEmuRegisters, opoffs: u32) -> u32 {
    let flag = opl_emu_registers_newflag(regs);
    let shift_amount = if flag > 0 { (flag as u32).wrapping_shr(1) * 2 } else { 0 };
    if shift_amount > 31 {
        return 0;
    }
    match regs.m_regdata.get(0xe0 as usize + opoffs as usize) {
        Some(val) => opl_emu_bitfield(*val as u32, 0, shift_amount),
        None => 0,
    }
}


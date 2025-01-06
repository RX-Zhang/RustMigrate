
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shr((start % 32) as u32)) & ((1u32.wrapping_shl(length as u32)) - 1)
}

fn opl_emu_registers_byte(
    regs: &Box<OplEmuRegisters>,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    let index = (offset.wrapping_add(extra_offset)) as usize;
    if index < regs.m_regdata.len() {
        opl_emu_bitfield(regs.m_regdata[index] as u32, start as i32, count as i32)
    } else {
        0 // Handle out-of-bounds access safely
    }
}

fn opl_emu_registers_op_eg_sustain(regs: &Box<OplEmuRegisters>, opoffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0x20, 5, 1, opoffs)
}

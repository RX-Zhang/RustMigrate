
use std::mem;

const OPL_EMU_ISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u16 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u8,
    // other fields
}

fn opl_emu_write_register(
    regs: &mut OplEmuRegisters,
    reg: u16,
    val: u8,
) {
    match reg {
        0x0 => regs.m_lfo_am_counter = ((regs.m_lfo_am_counter as u16) & 0x3ff) | ((val as u16) << 10),
        0x1 => regs.m_lfo_am_counter = (regs.m_lfo_am_counter & 0xc00) | (val as u16),
        0x2 => regs.m_lfo_pm_counter = val,
        _ => (),
    }
}

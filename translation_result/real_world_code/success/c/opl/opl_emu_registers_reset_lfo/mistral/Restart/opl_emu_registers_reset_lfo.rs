
use std::u16;
use std::u32;
use std::u8;

const OPL_U_REGISTERSWVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_U_REGISTERSWVEFORMS],
}

fn opl_emu_registers_reset_lfo(regs: &mut OplEmuRegisters) {
    regs.m_lfo_am_counter = 0u16;
    regs.m_lfo_pm_counter = 0u16;
}

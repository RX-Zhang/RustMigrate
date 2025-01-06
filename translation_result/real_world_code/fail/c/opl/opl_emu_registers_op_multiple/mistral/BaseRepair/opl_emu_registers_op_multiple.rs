
use std::mem;

const PL_EMU_REGERS_VEFORMS: usize =8;
const OPLEU_REGISTERSREGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u8, // changed to u8 to fix the error
}

impl Default for OplEmuRegisters {
    fn default() -> Self {
        OplEmuRegisters {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
        }
    }
}

fn set_lfo_am_counter(registers: &mut OplEmuRegisters, value: u16) {
    registers.m_lfo_am_counter = value;
}

fn get_lfo_am_counter(registers: &OplEmuRegisters) -> u16 {
    registers.m_lfo_am_counter
}

fn set_lfo_pm_counter(registers: &mut OplEmuRegisters, value: u8) {
    registers.m_lfo_pm_counter = value;
}

fn get_lfo_pm_counter(registers: &OplEmuRegisters) -> u8 {
    registers.m_lfo_pm_counter
}

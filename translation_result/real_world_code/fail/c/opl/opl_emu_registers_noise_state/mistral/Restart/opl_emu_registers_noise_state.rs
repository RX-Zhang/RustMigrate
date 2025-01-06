
use std::ops::Shr;

const OPL_EMUREGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 256; // 0x200 in hexadecimal
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 1024; // 0x400 in hexadecimal

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMUREGISTERS_WAVEFORMS],
}

impl Default for OplEmuRegisters {
    fn default() -> Self {
        // Initialize the structure with all zeros
        OplEmuRegisters {
            m_lfo_am_counter: 0,
            m_lfo_pm_counter: 0,
            m_noise_lfsr: 0,
            m_lfo_am: 0,
            m_regdata: [0; OPL_EMU_REGISTERS_REGISTERS],
            m_waveform: [[0; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMUREGISTERS_WAVEFORMS],
        }
    }
}

fn opl_emu_registers_noise_state(regs: &mut OplEmuRegisters) -> u32 {
    ((regs.m_noise_lfsr >> 23) & 0x1F) as u32
}

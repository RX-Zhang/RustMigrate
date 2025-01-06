
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16, // LFO AM counter
    m_lfo_pm_counter: u16, // LFO PM counter 
    m_noise_lfsr: u32, // noise LFSR state
    m_lfo_am: u8, // current LFO AM value
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS]>, // register data
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS]>, // waveforms
}

fn opl_emu_registers_reset(regs: &mut OplEmuRegisters) {
    for i in 0..OPL_EMU_REGISTERS_REGISTERS {
        regs.m_regdata[i] = 0;
    }
}



use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],
    m_waveform: [Box<[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]>; OPL_EMU_REGISTERS_WAVEFORMS as usize],
}

fn opl_emu_bitfield(value: u16, start: i32, length: i32) -> u16 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_abs_sin_attenuation(input: u16) -> u16 {
    0
}   

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    regs.m_lfo_am_counter = 0;
    regs.m_lfo_pm_counter = 0;
    regs.m_noise_lfsr = 1;
    regs.m_lfo_am = 0;

    // create waveforms
    for index in 0..OPL_EMU_REGISTERS_WAVEFORM_LENGTH {
        regs.m_waveform[0][index as usize] = opl_emu_abs_sin_attenuation(index as u16) 
            .wrapping_add(opl_emu_bitfield(index as u16, 9, 1) << 15);
    }
    
    // additional waveform initialization
}


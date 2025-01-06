

use std::boxed::Box;

#[derive(Debug)]
pub struct OplEmuRegisters {
    // internal state
    lfo_am_counter: u16,            // LFO AM counter
    lfo_pm_counter: u16,            // LFO PM counter
    noise_lfsr: u32,                // noise LFSR state
    lfo_am: u8,                     // current LFO AM value
    regdata: Box<[u8; 0x200]>,         // register data
    waveform: Box<[[u16; 0x400]; 8]>, // waveforms
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(
    regs: &mut OplEmuRegisters,
    offset: u32,
    start: u32,
    count: u32,
    extra_offset: u32,
) -> u32 {
    opl_emu_bitfield(
        regs.regdata[offset as usize + extra_offset as usize] as u32,
        start as i32,
        count as i32,
    )
}

fn opl_emu_registers_ch_feedback(regs: &mut OplEmuRegisters, choffs: u32) -> u32 {
    opl_emu_registers_byte(regs, 0xc0, 1, 3, choffs)
}


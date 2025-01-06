

use std::mem;

const OPL_EMU_REGISTERS_WAVEFORMS: u32 = 8;
const OPL_EMU_REGISTERS_REGISTERS: u32 = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: u32 = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,            // LFO AM counter
    m_lfo_pm_counter: u16,            // LFO PM counter
    m_noise_lfsr: u32,                // noise LFSR state
    m_lfo_am: u8,                     // current LFO AM value
    m_regdata: Box<[u8; OPL_EMU_REGISTERS_REGISTERS as usize]>,         // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize], // waveforms
}

fn memset(dest: &mut [u8], c: u8, n: usize) {
    for i in 0..n {
        dest[i] = c;
    }
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    let start = start as i32;
    let length = length as i32;
    (value.wrapping_shl(start as u32) & ((1 << length) - 1)) as u32
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset + extra_offset) as usize;
    let value = regs.m_regdata[index];
    opl_emu_bitfield(value as u32, start, count)
}

fn opl_emu_registers_newflag(regs: &mut OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

fn opl_emu_registers_ch_output_2(regs: &mut OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) != 0 {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 6, 1, 0)
    } else {
        0
    }
}

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    let mut regdata = regs.m_regdata.as_mut_slice();
    memset(&mut regdata, 0, mem::size_of::<u8>() * OPL_EMU_REGISTERS_REGISTERS as usize);
}


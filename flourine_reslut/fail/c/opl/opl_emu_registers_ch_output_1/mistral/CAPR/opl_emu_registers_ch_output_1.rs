

use libc::memset;
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
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS as usize],         // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize], // waveforms
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    if start < 0 || length < 0 || start as u32 >= 32 || (start as u32 + length as u32) > 32 {
        panic!("start or length is negative or exceeds 32");
    }
    (value >> (start as u32)) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
        let index = (offset + extra_offset) as usize;
        let value = regs.m_regdata[index];
        opl_emu_bitfield(value as u32, start as i32, count as i32)
    }

fn opl_emu_registers_newflag(regs: &mut OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

fn opl_emu_registers_ch_output_1(regs: &mut OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) != 0 {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 5, 1, 0)
    } else {
        1
    }
}

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    unsafe {
        memset(regs as *mut _ as *mut libc::c_void, 0, mem::size_of::<OplEmuRegisters>() as libc::size_t);
    }
}


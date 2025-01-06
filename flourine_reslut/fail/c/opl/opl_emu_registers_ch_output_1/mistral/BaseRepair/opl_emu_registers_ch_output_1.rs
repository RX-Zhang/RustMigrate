

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
    m_waveform: Box<[[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH as usize]; OPL_EMU_REGISTERS_WAVEFORMS as usize]>, // waveforms
}

extern "C" {
    fn memset(dest: *mut core::ffi::c_void, c: i32, n: u64) -> *mut core::ffi::c_void;
}

fn u32_to_i32(value: u32) -> i32 {
    value as i32
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value.wrapping_shl(start as u32) & ((1 << length) - 1)) as u32
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let index = (offset + extra_offset) as usize;
    let value = regs.m_regdata[index];
    opl_emu_bitfield(value as u32, u32_to_i32(start), u32_to_i32(count))
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
        memset(regs as *mut _ as *mut core::ffi::c_void, 0, mem::size_of::<OplEmuRegisters>() as u64);
    }
}


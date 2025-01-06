
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

extern "C" {
    fn memset(dest: *mut u8, c: i32, n: u64);
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &mut OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    let real_offset = offset as usize + extra_offset as usize;
    let value = regs.m_regdata[real_offset] as u32;
    opl_emu_bitfield(value, start, count)
}

fn opl_emu_registers_newflag(regs: &mut OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0x105, 0, 1, 0)
}

fn opl_emu_registers_ch_output_3(regs: &mut OplEmuRegisters, choffs: u32) -> u32 {
    if opl_emu_registers_newflag(regs) != 0 {
        opl_emu_registers_byte(regs, 0xc0 + choffs, 7, 1, 0)
    } else {
        0
    }
}

fn opl_emu_registers_init(regs: &mut OplEmuRegisters) {
    unsafe {
        memset(regs as *mut _ as *mut u8, 0, mem::size_of::<OplEmuRegisters>() as u64);
    }
}

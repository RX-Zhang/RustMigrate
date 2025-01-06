
use std::boxed::Box;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

struct OplEmuRegisters {
    // internal state
    m_lfo_am_counter: u16,           // LFO AM counter
    m_lfo_pm_counter: u16,           // LFO PM counter
    m_noise_lfsr: u32,               // noise LFSR state
    m_lfo_am: u8,                    // current LFO AM value
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],    // register data
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS], // waveforms
}

fn opl_emu_bitfield(value: u32, start: i32, length: i32) -> u32 {
    (value >> start.wrapping_sub(length) as u32) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: u32, start: u32, count: u32, extra_offset: u32) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset as usize + extra_offset as usize] as u32, start as i32, count as i32)
}

fn opl_emu_registers_lfo_pm_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 6, 1, 0)
}

fn opl_emu_opl_clock_noise_and_lfo(
    noise_lfsr: &mut u32,
    lfo_am_counter: &mut u16,
    lfo_pm_counter: &mut u16,
    lfo_am: &mut u8,
    am_depth: u32,
    pm_depth: u32) -> i32 {
    
    0
}

fn opl_emu_registers_clock_noise_and_lfo(regs: &mut OplEmuRegisters) -> i32 {
    let am_depth = opl_emu_registers_lfo_pm_depth(&*regs);
    let pm_depth = opl_emu_registers_lfo_pm_depth(&*regs);
    opl_emu_opl_clock_noise_and_lfo(
        &mut regs.m_noise_lfsr,
        &mut regs.m_lfo_am_counter,
        &mut regs.m_lfo_pm_counter,
        &mut regs.m_lfo_am,
        am_depth,
        pm_depth)
}


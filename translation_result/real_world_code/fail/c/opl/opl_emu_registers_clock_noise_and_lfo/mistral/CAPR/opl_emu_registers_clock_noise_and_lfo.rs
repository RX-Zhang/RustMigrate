
use std::u16;
use std::u32;

const OPL_EMU_REGISTERS_WAVEFORMS: usize = 8;
const OPL_EMU_REGISTERS_REGISTERS: usize = 0x200;
const OPL_EMU_REGISTERS_WAVEFORM_LENGTH: usize = 0x400;

#[repr(C)]
struct OplEmuRegisters {
    m_lfo_am_counter: u16,
    m_lfo_pm_counter: u16,
    m_noise_lfsr: u32,
    m_lfo_am: u8,
    m_regdata: [u8; OPL_EMU_REGISTERS_REGISTERS],
    m_waveform: [[u16; OPL_EMU_REGISTERS_WAVEFORM_LENGTH]; OPL_EMU_REGISTERS_WAVEFORMS],
}

fn opl_emu_bitfield(value: u32, start: u32, length: u32) -> u32 {
    (value >> start) & ((1 << length) - 1)
}

fn opl_emu_registers_byte(regs: &OplEmuRegisters, offset: usize, start: u32, count: u32, extra_offset: usize) -> u32 {
    opl_emu_bitfield(regs.m_regdata[offset + extra_offset] as u32, start, count)
}

fn opl_emu_opl_clock_noise_and_lfo(noise_lfsr: &mut u32, lfo_am_counter: &mut u16, lfo_pm_counter: &mut u16, lfo_am: &mut u8, am_depth: u32, pm_depth: u32) -> i32 {
    *noise_lfsr <<= 1;
    *noise_lfsr |= ((*noise_lfsr >> 23) ^ (*noise_lfsr >> 9) ^ (*noise_lfsr >> 8) ^ (*noise_lfsr >> 1)) as u32 & 1;
    let am_counter = *lfo_am_counter;
    if am_counter >= (210 * 64 - 1) as u16 {
        *lfo_am_counter = 0;
    }
    let shift = 9 - 2 * am_depth as u32;
    *lfo_am = ((am_counter < (105 * 64) as u16) as u8) << shift | ((am_counter < (105 * 64) as u16) as u8 >> (8 - shift));
    let pm_counter = *lfo_pm_counter;
    let pm_scale = [8, 4, 0, -4, -8, -4, 0, 4];
    let pm_depth_mask = pm_depth ^ 1;
    (pm_scale[pm_counter as usize & 7] >> (pm_depth_mask as usize)) as i32
}

fn opl_emu_registers_lfo_pm_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 6, 1, 0)
}

fn opl_emu_registers_lfo_am_depth(regs: &OplEmuRegisters) -> u32 {
    opl_emu_registers_byte(regs, 0xbd, 7, 1, 0)
}

fn opl_emu_registers_clock_noise_and_lfo(regs: &mut OplEmuRegisters) -> i32 {
    let am_depth = opl_emu_registers_lfo_am_depth(regs);
    let pm_depth = opl_emu_registers_lfo_pm_depth(regs);
    opl_emu_opl_clock_noise_and_lfo(&mut regs.m_noise_lfsr, &mut regs.m_lfo_am_counter, &mut regs.m_lfo_pm_counter, &mut regs.m_lfo_am, am_depth, pm_depth)
}
